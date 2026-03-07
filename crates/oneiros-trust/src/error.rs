use crate::Endpoint;

/// Errors that can occur during trust management operations.
#[derive(Debug, thiserror::Error)]
pub enum TrustError {
    #[error("failed to initialize CA: {0}")]
    CaInitFailed(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("certificate expired for {0}")]
    CertExpired(Endpoint),

    #[error("ACME certificate acquisition failed: {0}")]
    AcmeFailed(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("trust storage error: {0}")]
    StorageFailed(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("endpoint {0} is not trusted")]
    Untrusted(Endpoint),

    #[error("insecure connection to {0} refused — not in allowlist")]
    InsecureRefused(Endpoint),

    #[error("failed to install root CA to system trust store: {0}")]
    TrustStoreInstallFailed(String),

    #[error("TLS is not enabled — trust mode is Off")]
    TlsNotEnabled,

    #[error("ACME TLS server configuration is not yet available")]
    AcmeNotConfigured,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TLS error: {0}")]
    Tls(#[from] rustls::Error),
}
