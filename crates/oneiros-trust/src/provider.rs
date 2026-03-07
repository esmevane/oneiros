use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{ClientConfig, RootCertStore, ServerConfig};

use crate::{
    acme::AcmeServerState,
    ca::{LeafCert, LocalCa},
    TrustError,
};

/// Opaque server TLS configuration ready for use with an HTTP listener.
///
/// Variants correspond to the resolved trust mode:
///
/// - `Local` — rustls [`ServerConfig`] backed by the local self-signed CA.
/// - `Acme` — [`AcmeServerState`] holding the ACME challenge resolver and the
///   axum acceptor. The caller must drive the embedded `AcmeState` stream to
///   keep certificates current.
pub enum SecureServer {
    /// Static TLS configuration from the local CA leaf certificate.
    Local(Arc<ServerConfig>),
    /// ACME-managed TLS using Let's Encrypt / TLS-ALPN-01.
    Acme(AcmeServerState),
}

impl SecureServer {
    /// Build a [`SecureServer::Local`] directly from a [`LeafCert`].
    ///
    /// This is the right entry point when the calling code already holds a
    /// `LeafCert` and does not need any provider lifecycle.
    ///
    /// # Errors
    ///
    /// Returns [`TrustError`] if the PEM data cannot be parsed or if rustls
    /// rejects the certificate / key pair.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use oneiros_trust::{LocalCa, SecureServer};
    ///
    /// let dir = tempfile::TempDir::new().unwrap();
    /// let ca = LocalCa::init(dir.path()).unwrap();
    /// let leaf = ca.issue_leaf("localhost").unwrap();
    /// let server = SecureServer::local(&leaf).unwrap();
    /// ```
    pub fn local(leaf: &LeafCert) -> Result<Self, TrustError> {
        let certs = parse_cert_chain(&leaf.cert_pem)?;
        let key = parse_private_key(&leaf.key_pem)?;

        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;

        Ok(SecureServer::Local(Arc::new(config)))
    }
}

/// Opaque client TLS configuration built to trust the local CA root.
pub struct SecureClient {
    pub(crate) config: Arc<ClientConfig>,
}

impl SecureClient {
    /// Returns the underlying rustls [`ClientConfig`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use oneiros_trust::{LocalCa, SecureClient};
    ///
    /// let dir = tempfile::TempDir::new().unwrap();
    /// let ca = oneiros_trust::LocalCa::init(dir.path()).unwrap();
    /// let client = SecureClient::local(&ca).unwrap();
    /// let _config = client.client_config();
    /// ```
    pub fn client_config(&self) -> &Arc<ClientConfig> {
        &self.config
    }
}

impl SecureClient {
    /// Build a [`SecureClient`] that trusts the given local CA root.
    ///
    /// # Errors
    ///
    /// Returns [`TrustError`] if the CA's root certificate PEM cannot be
    /// parsed or if rustls rejects the certificate.
    pub fn local(ca: &LocalCa) -> Result<Self, TrustError> {
        let root_certs = parse_cert_chain(ca.root_cert_pem())?;
        let mut root_store = RootCertStore::empty();
        for cert in root_certs {
            root_store.add(cert)?;
        }

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        Ok(SecureClient {
            config: Arc::new(config),
        })
    }

    /// Build a [`SecureClient`] that trusts system / public roots (for ACME mode).
    pub fn acme() -> Self {
        let mut root_store = RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        SecureClient {
            config: Arc::new(config),
        }
    }
}

// ---------------------------------------------------------------------------
// PEM parsing helpers
// ---------------------------------------------------------------------------

/// Compute the SHA-256 fingerprint of the first DER certificate in a PEM string.
///
/// Returns a `"sha256:<hexlower>"` string, or `None` if the PEM contains no
/// parseable DER certificates.
///
/// # Examples
///
/// ```no_run
/// use oneiros_trust::{LocalCa, ca_fingerprint};
///
/// let dir = tempfile::TempDir::new().unwrap();
/// let ca = LocalCa::init(dir.path()).unwrap();
/// let fingerprint = ca_fingerprint(ca.root_cert_pem());
/// assert!(fingerprint.unwrap().starts_with("sha256:"));
/// ```
pub fn ca_fingerprint(pem: &str) -> Option<String> {
    use data_encoding::HEXLOWER;
    use sha2::{Digest, Sha256};

    let mut reader = std::io::BufReader::new(pem.as_bytes());
    let der_certs: Vec<_> = rustls_pemfile::certs(&mut reader)
        .filter_map(|r| r.ok())
        .collect();

    let der = der_certs.first()?;
    let digest = Sha256::digest(der.as_ref());
    Some(format!("sha256:{}", HEXLOWER.encode(&digest)))
}

pub(crate) fn parse_cert_chain(pem: &str) -> Result<Vec<CertificateDer<'static>>, TrustError> {
    let mut reader = BufReader::new(pem.as_bytes());
    let certs: Result<Vec<_>, _> = rustls_pemfile::certs(&mut reader).collect();
    certs.map_err(|source| TrustError::PemParseFailed {
        path: PathBuf::from("<in-memory>"),
        source,
    })
}

pub(crate) fn parse_private_key(pem: &str) -> Result<PrivateKeyDer<'static>, TrustError> {
    let mut reader = BufReader::new(pem.as_bytes());
    rustls_pemfile::private_key(&mut reader)
        .map_err(|source| TrustError::PemParseFailed {
            path: PathBuf::from("<in-memory>"),
            source,
        })?
        .ok_or(TrustError::NoPrivateKey)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secure_server_local_constructor_builds_from_leaf_cert() {
        let dir = tempfile::TempDir::new().unwrap();
        let ca = crate::LocalCa::init(dir.path()).unwrap();
        let leaf = ca.issue_leaf("localhost").unwrap();
        assert!(SecureServer::local(&leaf).is_ok());
    }

    #[test]
    fn ca_fingerprint_returns_sha256_prefixed_string() {
        let dir = tempfile::TempDir::new().unwrap();
        let ca = crate::LocalCa::init(dir.path()).unwrap();
        let fp = ca_fingerprint(ca.root_cert_pem());
        assert!(fp.is_some());
        assert!(fp.unwrap().starts_with("sha256:"));
    }

    #[test]
    fn ca_fingerprint_returns_none_for_empty_pem() {
        assert!(ca_fingerprint("").is_none());
    }

    #[test]
    fn secure_client_local_builds_from_ca() {
        let dir = tempfile::TempDir::new().unwrap();
        let ca = crate::LocalCa::init(dir.path()).unwrap();
        assert!(SecureClient::local(&ca).is_ok());
    }

    #[test]
    fn secure_client_acme_builds_without_local_ca() {
        let _client = SecureClient::acme();
    }
}
