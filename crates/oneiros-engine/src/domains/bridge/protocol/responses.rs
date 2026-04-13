use serde::{Deserialize, Serialize};

use crate::*;

/// The response to a [`SyncRequest`] over the oneiros sync protocol.
///
/// Carried over the `/oneiros/sync/1` ALPN via iroh's QUIC transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum BridgeResponse {
    /// Canon updates the requestor is missing (encoded Loro updates).
    Updates { canon_bytes: Vec<u8> },

    /// The requestor is already up to date — no updates needed.
    Current,

    /// The requested events, fetched by ID after a conference.
    Events { events: Vec<StoredEvent> },

    /// The server rejected the request. The reason is intended for human
    /// consumption on the client side (logging, error surfaces).
    Denied { reason: String },
}

impl BridgeResponse {
    /// Encode this response to JSON bytes for transport.
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("sync response serialization should not fail")
    }

    /// Decode a response from JSON bytes received over transport.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    /// Whether this response indicates the request was denied.
    pub(crate) fn is_denied(&self) -> bool {
        matches!(self, Self::Denied { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn updates_response_roundtrip() {
        let original = BridgeResponse::Updates {
            canon_bytes: vec![1, 2, 3, 4],
        };
        let bytes = original.to_bytes();
        let decoded = BridgeResponse::from_bytes(&bytes).unwrap();
        assert!(!decoded.is_denied());
        assert_eq!(decoded.to_bytes(), bytes);
        match decoded {
            BridgeResponse::Updates { canon_bytes } => assert_eq!(canon_bytes, vec![1, 2, 3, 4]),
            _ => panic!("expected Updates variant"),
        }
    }

    #[test]
    fn current_response_roundtrip() {
        let original = BridgeResponse::Current;
        let bytes = original.to_bytes();
        let decoded = BridgeResponse::from_bytes(&bytes).unwrap();
        assert!(!decoded.is_denied());
        assert!(matches!(decoded, BridgeResponse::Current));
    }

    #[test]
    fn denied_response_roundtrip() {
        let original = BridgeResponse::Denied {
            reason: "ticket expired".into(),
        };
        let bytes = original.to_bytes();
        let decoded = BridgeResponse::from_bytes(&bytes).unwrap();
        assert!(decoded.is_denied());
        match decoded {
            BridgeResponse::Denied { reason } => assert_eq!(reason, "ticket expired"),
            _ => panic!("expected Denied variant"),
        }
    }
}
