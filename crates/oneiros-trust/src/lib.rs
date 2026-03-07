pub mod acme;
mod ca;
mod error;
mod mode;
mod peers;
mod provider;
mod trust_store;
mod types;

pub use acme::{build_acme_state, AcmeServerState};
pub use ca::{LeafCert, LocalCa};
pub use error::TrustError;
pub use mode::resolve_mode;
pub use peers::PeerStore;
pub use provider::{SecureClient, SecureServer, TrustProvider};
pub use trust_store::{install_command, install_root_ca};
pub use types::*;

// Re-export TrustMode from the config crate as the canonical definition.
pub use oneiros_config::TrustMode;
