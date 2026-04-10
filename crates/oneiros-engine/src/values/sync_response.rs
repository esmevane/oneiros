use serde::{Deserialize, Serialize};

use crate::*;

/// The response to a [`SyncRequest`] over the oneiros sync protocol.
///
/// Carried over the `/oneiros/sync/1` ALPN via iroh's QUIC transport.
/// Encoded with postcard on the wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncResponse {
    /// The server accepted the request and returned these events. Empty vec
    /// is valid — it means the caller is already up to date.
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

    // StoredEvent doesn't implement PartialEq (its inner `Events` enum is
    // large and structural), so we round-trip responses through bytes and
    // compare re-serialized output rather than comparing structs directly.

    #[test]
    fn empty_events_response_roundtrip() {
        let original = SyncResponse::Events { events: Vec::new() };
        let bytes = original.to_bytes();
        let decoded = SyncResponse::from_bytes(&bytes).unwrap();
        assert!(!decoded.is_denied());
        assert_eq!(decoded.to_bytes(), bytes);
        match decoded {
            SyncResponse::Events { events } => assert!(events.is_empty()),
            _ => panic!("expected Events variant"),
        }
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

    #[test]
    fn denied_response_json_roundtrip() {
        let original = SyncResponse::Denied {
            reason: "no such bookmark".into(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let decoded: SyncResponse = serde_json::from_str(&json).unwrap();
        match decoded {
            SyncResponse::Denied { reason } => assert_eq!(reason, "no such bookmark"),
            _ => panic!("expected Denied variant"),
        }
    }
}
