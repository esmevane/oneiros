use serde::{Deserialize, Serialize};

use crate::*;

/// A request issued over the oneiros sync protocol. The single variant
/// `Pull` is the "give me events for this link since this checkpoint"
/// operation that powers `bookmark collect`.
///
/// Carried over the `/oneiros/sync/1` ALPN via iroh's QUIC transport.
/// Encoded with postcard on the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SyncRequest {
    /// Pull events matching the link's target, starting after the caller's
    /// last-known checkpoint. The link's token is validated against the
    /// server's tickets table before any events are returned.
    Pull { link: Link, checkpoint: Checkpoint },
}

impl SyncRequest {
    /// Encode this request to JSON bytes for transport. JSON is used
    /// over postcard because some types in the sync payload
    /// (particularly `Timestamp` / chrono) contain patterns postcard
    /// doesn't support.
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

    fn sample_pull() -> SyncRequest {
        SyncRequest::Pull {
            link: Link::new(Ref::bookmark(BookmarkId::new()), Token::from("testtoken")),
            checkpoint: Checkpoint::empty(),
        }
    }

    #[test]
    fn sync_request_roundtrip_through_bytes() {
        let original = sample_pull();
        let bytes = original.to_bytes();
        let decoded = SyncRequest::from_bytes(&bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn sync_request_roundtrip_through_serde_json() {
        let original = sample_pull();
        let json = serde_json::to_string(&original).unwrap();
        let decoded: SyncRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn sync_request_pull_matches() {
        let req = sample_pull();
        assert!(matches!(req, SyncRequest::Pull { .. }));
    }
}
