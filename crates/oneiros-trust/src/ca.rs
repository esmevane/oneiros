use std::{
    fs,
    path::{Path, PathBuf},
};

use rcgen::{
    BasicConstraints, CertificateParams, DnType, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use time::{Duration, OffsetDateTime};

use crate::TrustError;

/// A leaf TLS certificate and its private key, both PEM-encoded.
///
/// The `cert_pem` field contains the full chain: the leaf certificate
/// concatenated with the intermediate CA certificate. This is the format
/// expected by TLS servers.
///
/// # Examples
///
/// ```no_run
/// use oneiros_trust::LocalCa;
///
/// let dir = tempfile::TempDir::new().unwrap();
/// let ca = LocalCa::init(dir.path()).unwrap();
/// let leaf = ca.issue_leaf("localhost").unwrap();
/// assert!(!leaf.cert_pem.is_empty());
/// assert!(!leaf.key_pem.is_empty());
/// ```
pub struct LeafCert {
    /// Full chain PEM: leaf certificate + intermediate certificate.
    pub cert_pem: String,
    /// Private key PEM for the leaf certificate.
    pub key_pem: String,
}

/// A local two-tier certificate authority (root → intermediate) stored on disk.
///
/// The CA maintains a root certificate and an intermediate certificate. Leaf
/// certificates are signed by the intermediate. Only the root needs to be
/// installed into the system trust store.
///
/// # Examples
///
/// ```no_run
/// use oneiros_trust::LocalCa;
///
/// let dir = tempfile::TempDir::new().unwrap();
/// let trust_dir = dir.path().join("trust");
/// let ca = LocalCa::init(&trust_dir).unwrap();
/// println!("{}", ca.root_cert_pem());
/// ```
pub struct LocalCa {
    root_cert_pem: String,
    intermediate_cert_pem: String,
    intermediate_key_pem: String,
    trust_dir: PathBuf,
}

impl LocalCa {
    /// Initialize the CA: create root and intermediate if absent, load if present.
    ///
    /// Creates the full directory structure under `trust_dir`:
    /// - `certs/root.cert.pem`
    /// - `certs/root.key.pem`
    /// - `certs/intermediate.cert.pem`
    /// - `certs/intermediate.key.pem`
    /// - `leaves/` (empty, populated on demand)
    /// - `peers/` (empty, reserved for peer pinning)
    pub fn init(trust_dir: &Path) -> Result<Self, TrustError> {
        let certs_dir = trust_dir.join("certs");
        let root_cert_path = certs_dir.join("root.cert.pem");

        if root_cert_path.exists() {
            return Self::load(trust_dir);
        }

        fs::create_dir_all(&certs_dir)?;
        fs::create_dir_all(trust_dir.join("leaves"))?;
        fs::create_dir_all(trust_dir.join("peers"))?;

        let (root_cert_pem, root_key_pem, root_issuer) = generate_root_ca()?;
        let (intermediate_cert_pem, intermediate_key_pem) =
            generate_intermediate_ca(&root_issuer)?;

        fs::write(certs_dir.join("root.cert.pem"), &root_cert_pem)?;
        fs::write(certs_dir.join("root.key.pem"), &root_key_pem)?;
        fs::write(
            certs_dir.join("intermediate.cert.pem"),
            &intermediate_cert_pem,
        )?;
        fs::write(
            certs_dir.join("intermediate.key.pem"),
            &intermediate_key_pem,
        )?;

        Ok(Self {
            root_cert_pem,
            intermediate_cert_pem,
            intermediate_key_pem,
            trust_dir: trust_dir.to_owned(),
        })
    }

    /// Load an existing CA from PEM files on disk.
    ///
    /// Returns an error if any expected file is missing or malformed.
    pub fn load(trust_dir: &Path) -> Result<Self, TrustError> {
        let certs_dir = trust_dir.join("certs");

        let root_cert_pem = fs::read_to_string(certs_dir.join("root.cert.pem"))?;
        let intermediate_cert_pem = fs::read_to_string(certs_dir.join("intermediate.cert.pem"))?;
        let intermediate_key_pem = fs::read_to_string(certs_dir.join("intermediate.key.pem"))?;

        Ok(Self {
            root_cert_pem,
            intermediate_cert_pem,
            intermediate_key_pem,
            trust_dir: trust_dir.to_owned(),
        })
    }

    /// Returns the root CA certificate in PEM format.
    ///
    /// This is the certificate that must be installed into the system trust
    /// store for clients to trust leaf certificates issued by this CA.
    pub fn root_cert_pem(&self) -> &str {
        &self.root_cert_pem
    }

    /// Issue a leaf certificate for the given hostname, signed by the intermediate CA.
    ///
    /// The certificate includes the hostname as both the Common Name and a
    /// Subject Alternative Name. The resulting chain PEM (leaf + intermediate)
    /// and key PEM are written to `{trust_dir}/leaves/{hostname}.pem` and
    /// `{trust_dir}/leaves/{hostname}.key.pem` respectively.
    pub fn issue_leaf(&self, hostname: &str) -> Result<LeafCert, TrustError> {
        let intermediate_key = KeyPair::from_pem(&self.intermediate_key_pem)
            .map_err(|e| TrustError::CaInitFailed(e.into()))?;
        let issuer = Issuer::from_ca_cert_pem(&self.intermediate_cert_pem, intermediate_key)
            .map_err(|e| TrustError::CaInitFailed(e.into()))?;

        let (leaf_cert_pem, leaf_key_pem) = generate_leaf(hostname, &issuer)?;

        let chain_pem = format!("{}{}", leaf_cert_pem, self.intermediate_cert_pem);

        let leaves_dir = self.trust_dir.join("leaves");
        fs::create_dir_all(&leaves_dir)?;
        fs::write(leaves_dir.join(format!("{hostname}.pem")), &chain_pem)?;
        fs::write(
            leaves_dir.join(format!("{hostname}.key.pem")),
            &leaf_key_pem,
        )?;

        Ok(LeafCert {
            cert_pem: chain_pem,
            key_pem: leaf_key_pem,
        })
    }
}

/// Returns (cert_pem, key_pem, issuer) for a self-signed root CA.
fn generate_root_ca() -> Result<(String, String, Issuer<'static, KeyPair>), TrustError> {
    let mut params = CertificateParams::new(Vec::default())
        .map_err(|e| TrustError::CaInitFailed(e.into()))?;

    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params
        .distinguished_name
        .push(DnType::CommonName, "Oneiros Local Root CA");
    params.key_usages.push(KeyUsagePurpose::KeyCertSign);
    params.key_usages.push(KeyUsagePurpose::CrlSign);
    params.not_before = one_day_ago();
    params.not_after = ten_years_from_now();

    let key_pair = KeyPair::generate().map_err(|e| TrustError::CaInitFailed(e.into()))?;
    let cert = params
        .self_signed(&key_pair)
        .map_err(|e| TrustError::CaInitFailed(e.into()))?;

    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();
    let issuer = Issuer::new(params, key_pair);

    Ok((cert_pem, key_pem, issuer))
}

/// Returns (cert_pem, key_pem) for an intermediate CA signed by `root`.
fn generate_intermediate_ca(root: &Issuer<'static, KeyPair>) -> Result<(String, String), TrustError> {
    let mut params = CertificateParams::new(Vec::default())
        .map_err(|e| TrustError::CaInitFailed(e.into()))?;

    params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
    params
        .distinguished_name
        .push(DnType::CommonName, "Oneiros Local Intermediate CA");
    params.key_usages.push(KeyUsagePurpose::KeyCertSign);
    params.key_usages.push(KeyUsagePurpose::CrlSign);
    params.use_authority_key_identifier_extension = true;
    params.not_before = one_day_ago();
    params.not_after = ten_years_from_now();

    let key_pair = KeyPair::generate().map_err(|e| TrustError::CaInitFailed(e.into()))?;
    let cert = params
        .signed_by(&key_pair, root)
        .map_err(|e| TrustError::CaInitFailed(e.into()))?;

    Ok((cert.pem(), key_pair.serialize_pem()))
}

/// Returns (cert_pem, key_pem) for a leaf certificate signed by `issuer`.
fn generate_leaf(hostname: &str, issuer: &Issuer<'_, KeyPair>) -> Result<(String, String), TrustError> {
    let mut params = CertificateParams::new(vec![hostname.into()])
        .map_err(|e| TrustError::CaInitFailed(e.into()))?;

    params
        .distinguished_name
        .push(DnType::CommonName, hostname);
    params.use_authority_key_identifier_extension = true;
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ServerAuth);
    params.not_before = one_day_ago();
    params.not_after = one_year_from_now();

    let key_pair = KeyPair::generate().map_err(|e| TrustError::CaInitFailed(e.into()))?;
    let cert = params
        .signed_by(&key_pair, issuer)
        .map_err(|e| TrustError::CaInitFailed(e.into()))?;

    Ok((cert.pem(), key_pair.serialize_pem()))
}

fn one_day_ago() -> OffsetDateTime {
    OffsetDateTime::now_utc()
        .checked_sub(Duration::days(1))
        .expect("one day ago is always valid")
}

fn one_year_from_now() -> OffsetDateTime {
    OffsetDateTime::now_utc()
        .checked_add(Duration::days(365))
        .expect("one year from now is always valid")
}

fn ten_years_from_now() -> OffsetDateTime {
    OffsetDateTime::now_utc()
        .checked_add(Duration::days(365 * 10))
        .expect("ten years from now is always valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ca_init_creates_directory_structure() {
        let dir = tempfile::TempDir::new().unwrap();
        let trust_dir = dir.path().join("trust");
        LocalCa::init(&trust_dir).unwrap();
        assert!(trust_dir.join("certs/root.cert.pem").exists());
        assert!(trust_dir.join("certs/root.key.pem").exists());
        assert!(trust_dir.join("certs/intermediate.cert.pem").exists());
        assert!(trust_dir.join("certs/intermediate.key.pem").exists());
    }

    #[test]
    fn ca_load_reuses_existing() {
        let dir = tempfile::TempDir::new().unwrap();
        let trust_dir = dir.path().join("trust");
        let ca1 = LocalCa::init(&trust_dir).unwrap();
        let ca2 = LocalCa::load(&trust_dir).unwrap();
        assert_eq!(ca1.root_cert_pem(), ca2.root_cert_pem());
    }

    #[test]
    fn ca_generates_leaf_cert() {
        let dir = tempfile::TempDir::new().unwrap();
        let trust_dir = dir.path().join("trust");
        let ca = LocalCa::init(&trust_dir).unwrap();
        let leaf = ca.issue_leaf("localhost").unwrap();
        assert!(!leaf.cert_pem.is_empty());
        assert!(!leaf.key_pem.is_empty());
    }

    #[test]
    fn ca_leaf_persisted_to_leaves_dir() {
        let dir = tempfile::TempDir::new().unwrap();
        let trust_dir = dir.path().join("trust");
        let ca = LocalCa::init(&trust_dir).unwrap();
        ca.issue_leaf("localhost").unwrap();
        assert!(trust_dir.join("leaves/localhost.pem").exists());
    }
}
