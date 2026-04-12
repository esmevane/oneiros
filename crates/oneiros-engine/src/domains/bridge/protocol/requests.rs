use serde::{Deserialize, Serialize};

use crate::*;

/// A request issued over the oneiros sync protocol. The `Confer` variant
/// is the CRDT-native exchange that powers `bookmark collect` — the
/// requestor sends their version vector, and the peer responds with
/// only the canon updates they're missing.
///
/// Carried over the `/oneiros/sync/1` ALPN via iroh's QUIC transport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BridgeRequest {
    /// "Here's what I have — send me what I'm missing."
    ///
    /// The link's token is validated against the server's tickets table
    /// before any updates are returned. The version vector is the
    /// requestor's canon state (encoded `loro::VersionVector`).
    Confer { link: Link, version_vector: Vec<u8> },

    /// "Give me these specific events by ID."
    ///
    /// Issued after a conference, when the requestor has determined
    /// which events they need from the canon's event set diff.
    FetchEvents { link: Link, event_ids: Vec<String> },
}

impl BridgeRequest {
    /// Encode this request to JSON bytes for transport.
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("sync request serialization should not fail")
    }

    /// Decode a request from JSON bytes received over transport.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn sample_confer() -> BridgeRequest {
        BridgeRequest::Confer {
            link: Link::new(Ref::bookmark(BookmarkId::new()), Token::from("testtoken")),
            version_vector: vec![0, 1, 2, 3],
        }
    }

    #[test]
    fn sync_request_roundtrip_through_bytes() {
        let original = sample_confer();
        let bytes = original.to_bytes();
        let decoded = BridgeRequest::from_bytes(&bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn sync_request_roundtrip_through_serde_json() {
        let original = sample_confer();
        let json = serde_json::to_string(&original).unwrap();
        let decoded: BridgeRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn sync_request_confer_matches() {
        let req = sample_confer();
        assert!(matches!(req, BridgeRequest::Confer { .. }));
    }
}
