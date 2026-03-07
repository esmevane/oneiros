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
        write_key_file(certs_dir.join("root.key.pem"), &root_key_pem)?;
        fs::write(
            certs_dir.join("intermediate.cert.pem"),
            &intermediate_cert_pem,
        )?;
        write_key_file(
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
        let leaves_dir = self.trust_dir.join("leaves");
        let chain_path = leaves_dir.join(format!("{hostname}.pem"));
        let key_path = leaves_dir.join(format!("{hostname}.key.pem"));

        // Return existing leaf if both files are present — avoid unnecessary re-issuance.
        if chain_path.exists() && key_path.exists() {
            let cert_pem = fs::read_to_string(&chain_path)?;
            let key_pem = fs::read_to_string(&key_path)?;
            return Ok(LeafCert { cert_pem, key_pem });
        }

        let intermediate_key = KeyPair::from_pem(&self.intermediate_key_pem)
            .map_err(TrustError::CertGenerationFailed)?;
        let issuer = Issuer::from_ca_cert_pem(&self.intermediate_cert_pem, intermediate_key)
            .map_err(TrustError::CertGenerationFailed)?;

        let (leaf_cert_pem, leaf_key_pem) = generate_leaf(hostname, &issuer)?;

        let chain_pem = format!("{}{}", leaf_cert_pem, self.intermediate_cert_pem);

        fs::create_dir_all(&leaves_dir)?;
        fs::write(&chain_path, &chain_pem)?;
        write_key_file(&key_path, &leaf_key_pem)?;

        Ok(LeafCert {
            cert_pem: chain_pem,
            key_pem: leaf_key_pem,
        })
    }
}

#[cfg(unix)]
fn write_key_file(path: impl AsRef<Path>, contents: &str) -> Result<(), TrustError> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

#[cfg(not(unix))]
fn write_key_file(path: impl AsRef<Path>, contents: &str) -> Result<(), TrustError> {
    std::fs::write(path, contents)?;
    Ok(())
}

/// Returns (cert_pem, key_pem, issuer) for a self-signed root CA.
fn generate_root_ca() -> Result<(String, String, Issuer<'static, KeyPair>), TrustError> {
    let mut params = CertificateParams::new(Vec::default())
        .map_err(TrustError::CertGenerationFailed)?;

    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params
        .distinguished_name
        .push(DnType::CommonName, "Oneiros Local Root CA");
    params.key_usages.push(KeyUsagePurpose::KeyCertSign);
    params.key_usages.push(KeyUsagePurpose::CrlSign);
    params.not_before = one_day_ago();
    params.not_after = ten_years_from_now();

    let key_pair = KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256)
        .map_err(TrustError::CertGenerationFailed)?;
    let cert = params
        .self_signed(&key_pair)
        .map_err(TrustError::CertGenerationFailed)?;

    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();
    let issuer = Issuer::new(params, key_pair);

    Ok((cert_pem, key_pem, issuer))
}

/// Returns (cert_pem, key_pem) for an intermediate CA signed by `root`.
fn generate_intermediate_ca(root: &Issuer<'static, KeyPair>) -> Result<(String, String), TrustError> {
    let mut params = CertificateParams::new(Vec::default())
        .map_err(TrustError::CertGenerationFailed)?;

    params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
    params
        .distinguished_name
        .push(DnType::CommonName, "Oneiros Local Intermediate CA");
    params.key_usages.push(KeyUsagePurpose::KeyCertSign);
    params.key_usages.push(KeyUsagePurpose::CrlSign);
    params.use_authority_key_identifier_extension = true;
    params.not_before = one_day_ago();
    params.not_after = ten_years_from_now();

    let key_pair = KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256)
        .map_err(TrustError::CertGenerationFailed)?;
    let cert = params
        .signed_by(&key_pair, root)
        .map_err(TrustError::CertGenerationFailed)?;

    Ok((cert.pem(), key_pair.serialize_pem()))
}

/// Returns (cert_pem, key_pem) for a leaf certificate signed by `issuer`.
fn generate_leaf(hostname: &str, issuer: &Issuer<'_, KeyPair>) -> Result<(String, String), TrustError> {
    let mut params = CertificateParams::new(vec![hostname.into()])
        .map_err(TrustError::CertGenerationFailed)?;

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

    let key_pair = KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256)
        .map_err(TrustError::CertGenerationFailed)?;
    let cert = params
        .signed_by(&key_pair, issuer)
        .map_err(TrustError::CertGenerationFailed)?;

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

    #[cfg(unix)]
    #[test]
    fn key_files_have_restricted_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::TempDir::new().unwrap();
        let trust_dir = dir.path().join("trust");
        LocalCa::init(&trust_dir).unwrap();

        let root_key = trust_dir.join("certs/root.key.pem");
        let intermediate_key = trust_dir.join("certs/intermediate.key.pem");

        let root_mode = std::fs::metadata(&root_key).unwrap().permissions().mode() & 0o777;
        let int_mode = std::fs::metadata(&intermediate_key).unwrap().permissions().mode() & 0o777;

        assert_eq!(root_mode, 0o600, "root key should be 0o600, got {root_mode:o}");
        assert_eq!(int_mode, 0o600, "intermediate key should be 0o600, got {int_mode:o}");
    }

    #[cfg(unix)]
    #[test]
    fn leaf_key_files_have_restricted_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::TempDir::new().unwrap();
        let trust_dir = dir.path().join("trust");
        let ca = LocalCa::init(&trust_dir).unwrap();
        ca.issue_leaf("localhost").unwrap();

        let leaf_key = trust_dir.join("leaves/localhost.key.pem");
        let mode = std::fs::metadata(&leaf_key).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "leaf key should be 0o600, got {mode:o}");
    }

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

    #[test]
    fn issue_leaf_is_idempotent_when_leaf_exists() {
        let dir = tempfile::TempDir::new().unwrap();
        let trust_dir = dir.path().join("trust");
        let ca = LocalCa::init(&trust_dir).unwrap();

        let leaf1 = ca.issue_leaf("localhost").unwrap();
        let leaf2 = ca.issue_leaf("localhost").unwrap();

        // Second call should return the existing leaf, not regenerate.
        assert_eq!(leaf1.cert_pem, leaf2.cert_pem);
    }
}
