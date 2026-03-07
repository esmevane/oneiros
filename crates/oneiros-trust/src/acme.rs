use std::fmt::Debug;
use std::path::Path;
use std::sync::Arc;

use oneiros_config::AcmeConfig as AcmeConfigInput;
use rustls::ServerConfig;
use tokio_rustls_acme::caches::DirCache;
use tokio_rustls_acme::axum::AxumAcceptor;
use tokio_rustls_acme::{AcmeConfig, AcmeState};

use crate::TrustError;

const LETS_ENCRYPT_STAGING: &str =
    "https://acme-staging-v02.api.letsencrypt.org/directory";
const LETS_ENCRYPT_PRODUCTION: &str =
    "https://acme-v02.api.letsencrypt.org/directory";

/// ACME-managed TLS state for certificate provisioning via Let's Encrypt.
///
/// Wraps a [`tokio_rustls_acme::AcmeState`] (with boxed error types for
/// type-erasure) and a matching [`AxumAcceptor`] that handles both ACME
/// TLS-ALPN-01 challenges and normal TLS connections on the same port.
///
/// The [`AcmeState`] is a `Stream` that drives certificate acquisition and
/// renewal. The consumer (typically the HTTP server) must poll it to keep
/// certificates current — see [`AcmeServerState::into_parts`].
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use oneiros_config::AcmeConfig;
/// use oneiros_trust::acme::{build_acme_state, AcmeServerState};
///
/// let config = AcmeConfig {
///     contact: Some("mailto:admin@example.com".into()),
///     directory: None,
/// };
/// let dir = tempfile::TempDir::new().unwrap();
/// let state: AcmeServerState = build_acme_state(
///     &config,
///     vec!["example.com".into()],
///     dir.path(),
/// )
/// .unwrap();
/// ```
pub struct AcmeServerState {
    /// Stream-based state machine driving ACME cert acquisition/renewal.
    pub(crate) acme_state: AcmeState<Box<dyn Debug>>,
    /// Acceptor for use with `axum_server` — handles ACME challenges inline.
    pub(crate) acceptor: AxumAcceptor,
}

impl AcmeServerState {
    /// Decompose into the raw parts for use with an HTTP server.
    ///
    /// - The first element is the `AcmeState` stream. Spawn a task to poll it
    ///   so that certificate acquisition and renewal happen in the background.
    /// - The second element is the `AxumAcceptor`. Pass it to
    ///   `axum_server::bind(..).acceptor(acceptor)`.
    pub fn into_parts(self) -> (AcmeState<Box<dyn Debug>>, AxumAcceptor) {
        (self.acme_state, self.acceptor)
    }
}

/// Build ACME state for TLS certificate management via Let's Encrypt.
///
/// Produces an [`AcmeServerState`] containing:
/// - An `AcmeState` stream that drives cert acquisition (must be polled).
/// - An `AxumAcceptor` ready for use with `axum-server`.
///
/// The cache directory is set to `{cache_dir}/trust/leaves/`. The directory
/// is created if it does not exist.
///
/// # Directory selection
///
/// - If `config.directory` is `Some`, that URL is used directly.
/// - If `config.directory` is `None`, the Let's Encrypt **staging** directory
///   is used (safe for development; avoids rate limits).
///
/// # Errors
///
/// Returns [`TrustError::Io`] if the cache directory cannot be created.
pub fn build_acme_state(
    config: &AcmeConfigInput,
    domains: Vec<String>,
    cache_dir: &Path,
) -> Result<AcmeServerState, TrustError> {
    let leaf_cache_dir = cache_dir.join("trust").join("leaves");
    std::fs::create_dir_all(&leaf_cache_dir)?;

    let directory_url = config
        .directory
        .as_deref()
        .unwrap_or(LETS_ENCRYPT_STAGING);

    if directory_url == LETS_ENCRYPT_STAGING {
        tracing::warn!(
            "Using ACME staging directory — certificates will NOT be trusted by browsers. \
             Set [trust.acme.directory] to the production URL for real certificates."
        );
    }

    let is_production = directory_url == LETS_ENCRYPT_PRODUCTION;

    let mut acme_cfg = AcmeConfig::new(domains)
        .directory_lets_encrypt(is_production)
        .cache_with_boxed_err(DirCache::new(leaf_cache_dir));

    // Override with an explicit custom directory if one was provided that
    // isn't a standard Let's Encrypt URL.
    if !matches!(
        directory_url,
        LETS_ENCRYPT_STAGING | LETS_ENCRYPT_PRODUCTION
    ) {
        acme_cfg = acme_cfg.directory(directory_url);
    }

    if let Some(contact) = &config.contact {
        acme_cfg = acme_cfg.contact_push(contact.as_str());
    }

    let acme_state = acme_cfg.state();

    let rustls_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_cert_resolver(acme_state.resolver());

    let acceptor = acme_state.axum_acceptor(Arc::new(rustls_config));

    Ok(AcmeServerState {
        acme_state,
        acceptor,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use oneiros_config::AcmeConfig;

    #[test]
    fn acme_config_builds_state_with_staging() {
        let config = AcmeConfig {
            contact: Some("mailto:test@example.com".into()),
            directory: Some(LETS_ENCRYPT_STAGING.into()),
        };
        let dir = tempfile::TempDir::new().unwrap();
        let state = build_acme_state(&config, vec!["example.com".into()], dir.path());
        assert!(state.is_ok(), "failed to build ACME state: {:?}", state.err());
    }

    #[test]
    fn acme_config_builds_state_without_contact() {
        let config = AcmeConfig {
            contact: None,
            directory: None,
        };
        let dir = tempfile::TempDir::new().unwrap();
        let state = build_acme_state(&config, vec!["example.com".into()], dir.path());
        assert!(state.is_ok());
    }

    #[test]
    fn acme_config_creates_cache_directory() {
        let config = AcmeConfig {
            contact: Some("mailto:test@example.com".into()),
            directory: None,
        };
        let dir = tempfile::TempDir::new().unwrap();
        build_acme_state(&config, vec!["example.com".into()], dir.path()).unwrap();
        assert!(dir.path().join("trust").join("leaves").exists());
    }

    #[test]
    fn into_parts_yields_state_and_acceptor() {
        let config = AcmeConfig {
            contact: Some("mailto:test@example.com".into()),
            directory: None,
        };
        let dir = tempfile::TempDir::new().unwrap();
        let server_state =
            build_acme_state(&config, vec!["example.com".into()], dir.path()).unwrap();
        let (_acme_state, _acceptor) = server_state.into_parts();
        // Verify decomposition compiles and is structurally sound.
    }
}
