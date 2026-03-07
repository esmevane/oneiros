pub mod acme;
mod ca;
mod error;
mod mode;
mod provider;
mod trust_store;
mod types;

pub use acme::{build_acme_state, AcmeServerState};
pub use ca::{LeafCert, LocalCa};
pub use error::TrustError;
pub use mode::resolve_mode;
pub use provider::{SecureClient, SecureServer, ca_fingerprint};
pub use trust_store::{SystemTrustStore, TrustStoreBackend};
pub use types::*;

// Re-export TrustMode from the config crate as the canonical definition.
pub use oneiros_config::TrustMode;
