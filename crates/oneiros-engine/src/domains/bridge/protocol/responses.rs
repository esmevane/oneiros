use serde::{Deserialize, Serialize};

use crate::*;

/// The response to a [`SyncRequest`] over the oneiros sync protocol.
///
/// Carried over the `/oneiros/sync/1` ALPN via iroh's QUIC transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncResponse {
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

impl SyncResponse {
    /// Encode this response to JSON bytes for transport.
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("sync response serialization should not fail")
    }

    /// Decode a response from JSON bytes received over transport.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    /// Whether this response indicates the request was denied.
    pub fn is_denied(&self) -> bool {
        matches!(self, Self::Denied { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn updates_response_roundtrip() {
        let original = SyncResponse::Updates {
            canon_bytes: vec![1, 2, 3, 4],
        };
        let bytes = original.to_bytes();
        let decoded = SyncResponse::from_bytes(&bytes).unwrap();
        assert!(!decoded.is_denied());
        assert_eq!(decoded.to_bytes(), bytes);
        match decoded {
            SyncResponse::Updates { canon_bytes } => assert_eq!(canon_bytes, vec![1, 2, 3, 4]),
            _ => panic!("expected Updates variant"),
        }
    }

    #[test]
    fn current_response_roundtrip() {
        let original = SyncResponse::Current;
        let bytes = original.to_bytes();
        let decoded = SyncResponse::from_bytes(&bytes).unwrap();
        assert!(!decoded.is_denied());
        assert!(matches!(decoded, SyncResponse::Current));
    }

    #[test]
    fn denied_response_roundtrip() {
        let original = SyncResponse::Denied {
            reason: "ticket expired".into(),
        };
        let bytes = original.to_bytes();
        let decoded = SyncResponse::from_bytes(&bytes).unwrap();
        assert!(decoded.is_denied());
        match decoded {
            SyncResponse::Denied { reason } => assert_eq!(reason, "ticket expired"),
            _ => panic!("expected Denied variant"),
        }
    }
}
