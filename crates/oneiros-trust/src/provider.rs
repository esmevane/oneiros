use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use oneiros_config::{AcmeConfig as AcmeConfigInput, TrustConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{ClientConfig, RootCertStore, ServerConfig};

use crate::{
    acme::{build_acme_state, AcmeServerState},
    ca::{LeafCert, LocalCa},
    mode::resolve_mode,
    peers::PeerStore,
    trust_store,
    CaStatus, Endpoint, Fingerprint, LeafStatus, ResolvedMode, TrustError, TrustHealth, TrustStatus,
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
    /// use std::path::Path;
    /// use oneiros_config::TrustConfig;
    /// use oneiros_trust::TrustProvider;
    ///
    /// let dir = tempfile::TempDir::new().unwrap();
    /// let config = TrustConfig::default();
    /// let provider = TrustProvider::init(&config, dir.path(), "localhost").unwrap();
    /// let secure_client = provider.client().unwrap();
    /// let _config = secure_client.client_config();
    /// ```
    pub fn client_config(&self) -> &Arc<ClientConfig> {
        &self.config
    }
}

/// The central public API that orchestrates all trust components.
///
/// `TrustProvider` initialises the CA, issues leaf certificates, manages
/// peer state, and vends ready-to-use rustls configurations.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use oneiros_config::TrustConfig;
/// use oneiros_trust::TrustProvider;
///
/// let dir = tempfile::TempDir::new().unwrap();
/// let config = TrustConfig::default();
/// let provider = TrustProvider::init(&config, dir.path(), "localhost").unwrap();
/// let health = provider.health();
/// println!("{:?}", health.mode);
/// ```
pub struct TrustProvider {
    mode: ResolvedMode,
    ca: Option<LocalCa>,
    peers: PeerStore,
    trust_dir: PathBuf,
    data_dir: PathBuf,
    hostname: String,
    acme_config: AcmeConfigInput,
    trust_store_installed: bool,
    leaf: Option<LeafCert>,
}

impl TrustProvider {
    /// Initialise the trust provider from configuration.
    ///
    /// - `Local` mode (or `Auto` on a local hostname): initialises the CA,
    ///   issues a leaf certificate, and attempts to install the root CA into
    ///   the system trust store (failures are logged as warnings, not errors).
    /// - `Acme` mode: no local CA; ACME wiring is done in a later task.
    /// - `Off` mode: no CA, no certificates.
    ///
    /// The peer store is always loaded and seeded from the config's `peers`
    /// and `insecure` lists.
    pub fn init(
        config: &TrustConfig,
        data_dir: &Path,
        hostname: &str,
    ) -> Result<Self, TrustError> {
        let mode = resolve_mode(&config.mode, hostname);
        let trust_dir = data_dir.join("trust");
        let peers_dir = trust_dir.join("peers");

        std::fs::create_dir_all(&peers_dir)?;

        let mut peers = PeerStore::load(&peers_dir)?;
        seed_peers(&mut peers, config)?;

        let (ca, leaf) = match &mode {
            ResolvedMode::Local => init_local(&trust_dir, hostname)?,
            ResolvedMode::Acme | ResolvedMode::Off => (None, None),
        };

        Ok(Self {
            mode,
            ca,
            peers,
            trust_dir,
            data_dir: data_dir.to_owned(),
            hostname: hostname.to_owned(),
            acme_config: config.acme.clone(),
            trust_store_installed: false,
            leaf,
        })
    }

    /// Build server TLS configuration ready for use with a TLS listener.
    ///
    /// - `Local` mode: returns [`SecureServer::Local`] backed by the local CA
    ///   leaf certificate.
    /// - `Acme` mode: returns [`SecureServer::Acme`] with ACME state and an
    ///   axum acceptor. The caller must drive the embedded `AcmeState` stream.
    /// - `Off` mode: returns [`TrustError::TlsNotEnabled`].
    pub fn server(&self) -> Result<SecureServer, TrustError> {
        match &self.mode {
            ResolvedMode::Off => Err(TrustError::TlsNotEnabled),
            ResolvedMode::Acme => {
                let domains = vec![self.hostname.clone()];
                let state =
                    build_acme_state(&self.acme_config, domains, &self.data_dir)?;
                Ok(SecureServer::Acme(state))
            }
            ResolvedMode::Local => {
                let leaf = self.leaf.as_ref().ok_or_else(|| {
                    TrustError::CaInitFailed("leaf certificate not present".into())
                })?;

                let certs = parse_cert_chain(&leaf.cert_pem)?;
                let key = parse_private_key(&leaf.key_pem)?;

                let config = ServerConfig::builder()
                    .with_no_client_auth()
                    .with_single_cert(certs, key)?;

                Ok(SecureServer::Local(Arc::new(config)))
            }
        }
    }

    /// Build a rustls [`ClientConfig`] for outbound TLS connections.
    ///
    /// - `Local` mode: trusts only the local CA root certificate.
    /// - `Acme` mode: trusts system roots (via `webpki-roots`).
    /// - `Off` mode: returns [`TrustError::TlsNotEnabled`].
    pub fn client(&self) -> Result<SecureClient, TrustError> {
        match &self.mode {
            ResolvedMode::Off => Err(TrustError::TlsNotEnabled),
            ResolvedMode::Acme => {
                let mut root_store = RootCertStore::empty();
                root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
                let config = ClientConfig::builder()
                    .with_root_certificates(root_store)
                    .with_no_client_auth();
                Ok(SecureClient {
                    config: Arc::new(config),
                })
            }
            ResolvedMode::Local => {
                let ca = self
                    .ca
                    .as_ref()
                    .ok_or_else(|| TrustError::CaInitFailed("CA not initialised".into()))?;

                let root_certs = parse_cert_chain(ca.root_cert_pem())?;
                let mut root_store = RootCertStore::empty();
                for cert in root_certs {
                    root_store
                        .add(cert)
                        .map_err(|e| TrustError::CaInitFailed(e.into()))?;
                }

                let config = ClientConfig::builder()
                    .with_root_certificates(root_store)
                    .with_no_client_auth();
                Ok(SecureClient {
                    config: Arc::new(config),
                })
            }
        }
    }

    /// Attempt to install the local CA root certificate into the system trust store.
    ///
    /// This is an explicit, user-initiated action — it is NOT called automatically
    /// during `init()`. On macOS this will prompt for keychain access.
    ///
    /// Returns `Ok(true)` if installed successfully, `Ok(false)` if no CA exists
    /// (e.g. Off or Acme mode), or `Err` with an actionable message on failure.
    pub fn install_trust_store(&mut self) -> Result<bool, TrustError> {
        let Some(ref ca) = self.ca else {
            return Ok(false);
        };

        let root_cert_path = self.trust_dir.join("certs").join("root.cert.pem");
        // Ensure the cert is on disk (it should be from init, but be safe)
        if !root_cert_path.exists() {
            std::fs::write(&root_cert_path, ca.root_cert_pem())?;
        }

        let result = trust_store::install_root_ca(root_cert_path.to_string_lossy().as_ref())?;
        self.trust_store_installed = result;
        Ok(result)
    }

    /// Return the trust status of an endpoint from the peer store.
    pub fn status(&self, endpoint: &Endpoint) -> TrustStatus {
        self.peers.status(endpoint)
    }

    /// Accept a peer by recording its certificate fingerprint.
    pub fn accept_peer(
        &mut self,
        endpoint: &Endpoint,
        fingerprint: Fingerprint,
    ) -> Result<(), TrustError> {
        self.peers.accept(endpoint, fingerprint)
    }

    /// Add an endpoint to the insecure (non-TLS) allowlist.
    pub fn allow_insecure(&mut self, endpoint: &Endpoint) -> Result<(), TrustError> {
        self.peers.allow_insecure(endpoint)
    }

    /// Build a full health report for this node's trust state.
    pub fn health(&self) -> TrustHealth {
        let ca_status = ca_status(&self.ca);
        let leaf_status = leaf_status(&self.leaf);
        let known_peers = self.peers.known_peers();

        TrustHealth {
            ca_status,
            trust_store_installed: self.trust_store_installed,
            leaf_status,
            mode: self.mode.clone(),
            known_peers,
            trust_dir: self.trust_dir.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Init helpers
// ---------------------------------------------------------------------------

fn init_local(
    trust_dir: &Path,
    hostname: &str,
) -> Result<(Option<LocalCa>, Option<LeafCert>), TrustError> {
    let ca = LocalCa::init(trust_dir)?;
    let leaf = ca.issue_leaf(hostname)?;

    Ok((Some(ca), Some(leaf)))
}

fn seed_peers(peers: &mut PeerStore, config: &TrustConfig) -> Result<(), TrustError> {
    for peer in &config.peers {
        let endpoint = Endpoint::from(peer.endpoint.as_str());
        let fingerprint = Fingerprint::from(peer.fingerprint.clone());
        peers.accept(&endpoint, fingerprint)?;
    }

    for insecure in &config.insecure {
        let endpoint = Endpoint::from(insecure.endpoint.as_str());
        peers.allow_insecure(&endpoint)?;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// PEM parsing helpers
// ---------------------------------------------------------------------------

fn parse_cert_chain(pem: &str) -> Result<Vec<CertificateDer<'static>>, TrustError> {
    let mut reader = BufReader::new(pem.as_bytes());
    let certs: Result<Vec<_>, _> = rustls_pemfile::certs(&mut reader).collect();
    certs.map_err(|e| TrustError::CaInitFailed(e.into()))
}

fn parse_private_key(pem: &str) -> Result<PrivateKeyDer<'static>, TrustError> {
    let mut reader = BufReader::new(pem.as_bytes());
    rustls_pemfile::private_key(&mut reader)
        .map_err(|e| TrustError::CaInitFailed(e.into()))?
        .ok_or_else(|| TrustError::CaInitFailed("no private key found in PEM".into()))
}

// ---------------------------------------------------------------------------
// Status helpers
// ---------------------------------------------------------------------------

fn ca_status(ca: &Option<LocalCa>) -> CaStatus {
    if ca.is_some() {
        CaStatus::Valid
    } else {
        CaStatus::Missing
    }
}

fn leaf_status(leaf: &Option<LeafCert>) -> LeafStatus {
    if leaf.is_some() {
        LeafStatus::Valid
    } else {
        LeafStatus::Missing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oneiros_config::TrustConfig;

    fn local_config() -> TrustConfig {
        let mut config = TrustConfig::default();
        config.mode = oneiros_config::TrustMode::Local;
        config
    }

    #[test]
    fn init_creates_ca_in_local_mode() {
        let dir = tempfile::TempDir::new().unwrap();
        let provider = TrustProvider::init(&local_config(), dir.path(), "localhost").unwrap();
        let health = provider.health();
        assert_eq!(health.ca_status, CaStatus::Valid);
        assert_eq!(health.mode, ResolvedMode::Local);
    }

    #[test]
    fn init_off_mode_skips_ca() {
        let dir = tempfile::TempDir::new().unwrap();
        let config = TrustConfig::default(); // default is Off
        let provider = TrustProvider::init(&config, dir.path(), "localhost").unwrap();
        let health = provider.health();
        assert_eq!(health.ca_status, CaStatus::Missing);
        assert_eq!(health.mode, ResolvedMode::Off);
    }

    #[test]
    fn init_does_not_install_trust_store() {
        let dir = tempfile::TempDir::new().unwrap();
        let provider = TrustProvider::init(&local_config(), dir.path(), "localhost").unwrap();
        assert!(!provider.health().trust_store_installed);
    }

    #[test]
    fn server_returns_ok_in_local_mode() {
        let dir = tempfile::TempDir::new().unwrap();
        let provider = TrustProvider::init(&local_config(), dir.path(), "localhost").unwrap();
        assert!(provider.server().is_ok());
    }

    #[test]
    fn server_returns_err_in_off_mode() {
        let dir = tempfile::TempDir::new().unwrap();
        let config = TrustConfig::default(); // default is Off
        let provider = TrustProvider::init(&config, dir.path(), "localhost").unwrap();
        assert!(provider.server().is_err());
    }

    #[test]
    fn server_returns_ok_in_acme_mode() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut config = TrustConfig::default();
        config.mode = oneiros_config::TrustMode::Acme;
        config.acme.contact = Some("mailto:test@example.com".into());
        let provider = TrustProvider::init(&config, dir.path(), "brain.example.com").unwrap();
        assert!(provider.server().is_ok());
    }

    #[test]
    fn health_reports_leaf_status() {
        let dir = tempfile::TempDir::new().unwrap();
        let provider = TrustProvider::init(&local_config(), dir.path(), "localhost").unwrap();
        let health = provider.health();
        assert_eq!(health.leaf_status, LeafStatus::Valid);
    }
}
