use std::path::PathBuf;

use crate::Endpoint;

/// Errors that can occur during trust management operations.
#[derive(Debug, thiserror::Error)]
pub enum TrustError {
    // CA lifecycle
    #[error("failed to generate certificate: {0}")]
    CertGenerationFailed(#[source] rcgen::Error),

    #[error("failed to parse PEM from {path}: {source}")]
    PemParseFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("no private key found in PEM data")]
    NoPrivateKey,

    #[error("CA not initialized — run `oneiros trust init` first")]
    CaNotInitialized,

    #[error("leaf certificate not present for this mode")]
    LeafNotPresent,

    // TLS configuration
    #[error("TLS error: {0}")]
    Tls(#[from] rustls::Error),

    #[error("TLS is not enabled — trust mode is Off")]
    TlsNotEnabled,

    // Trust store
    #[error("failed to install root CA to system trust store: {0}")]
    TrustStoreInstallFailed(String),

    // ACME
    #[error("ACME configuration error: {0}")]
    AcmeConfigError(String),

    // Peer trust
    #[error("certificate expired for {0}")]
    CertExpired(Endpoint),

    #[error("endpoint {0} is not trusted")]
    Untrusted(Endpoint),

    #[error("insecure connection to {0} refused — not in allowlist")]
    InsecureRefused(Endpoint),

    // Storage / IO
    #[error("trust storage error: {0}")]
    StorageFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
