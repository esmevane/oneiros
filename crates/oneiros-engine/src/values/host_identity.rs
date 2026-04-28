use serde::{Deserialize, Serialize};

use crate::*;

/// Everything a host identifies itself as: its cryptographic key and its
/// current network reachability. Composed at runtime from the persisted
/// host keypair (generated at `oneiros system init`) and the bound iroh
/// `Endpoint`'s address.
///
/// Accessed via `ServerState::host_identity()` when composing outgoing
/// `oneiros://` URIs from shared bookmarks.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostIdentity {
    /// The host's ed25519 public key — stable across restarts.
    pub key: PeerKey,
    /// The host's current iroh endpoint address — reachability info that may
    /// change over time as the host's network environment changes.
    pub address: PeerAddress,
}

impl HostIdentity {
    pub fn new(key: PeerKey, address: PeerAddress) -> Self {
        Self { key, address }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn sample_identity() -> HostIdentity {
        let secret = iroh::SecretKey::generate();
        let endpoint_id = secret.public();
        let address = PeerAddress::new(iroh::EndpointAddr::new(endpoint_id));
        let key = PeerKey::from_bytes(*endpoint_id.as_bytes());
        HostIdentity::new(key, address)
    }

    #[test]
    fn host_identity_roundtrip_through_serde() {
        let original = sample_identity();
        let json = serde_json::to_string(&original).unwrap();
        let decoded: HostIdentity = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }
}
