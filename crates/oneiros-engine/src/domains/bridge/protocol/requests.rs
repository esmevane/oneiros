use serde::{Deserialize, Serialize};

use crate::*;

/// A diff request — "here's my chronicle root, tell me yours."
///
/// The link's token is validated against the server's tickets table
/// before any data is returned. The root hash is the requestor's
/// chronicle state for the bookmark being collected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BridgeDiff {
    pub link: Link,
    pub root_hash: Option<ContentHash>,
}

/// A resolve request — "give me these HAMT nodes by hash."
///
/// Issued during the Merkle tree walk when the client encounters
/// server node hashes it doesn't have locally. The server looks
/// them up in its ChronicleStore and returns the nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BridgeResolve {
    pub link: Link,
    pub hashes: Vec<ContentHash>,
}

/// A fetch request — "give me these specific events by ID."
///
/// Issued after the Merkle diff has identified which events the
/// client is missing. The server retrieves the full StoredEvents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BridgeFetchEvents {
    pub link: Link,
    pub event_ids: Vec<String>,
}

/// A request issued over the oneiros sync protocol.
///
/// Carried over the `/oneiros/sync/1` ALPN via iroh's QUIC transport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BridgeRequest {
    /// "Here's my chronicle root — tell me yours."
    ///
    /// Initiates the Merkle diff protocol. The server compares
    /// root hashes and responds with its root node if they differ.
    BridgeDiff(BridgeDiff),

    /// "Give me these HAMT nodes by hash."
    ///
    /// The client-driven tree walk requests server nodes it hasn't
    /// seen. Each round narrows the search by 16x (one HAMT level).
    BridgeResolve(BridgeResolve),

    /// "Give me these specific events by ID."
    ///
    /// Issued after the Merkle diff identifies missing events.
    BridgeFetchEvents(BridgeFetchEvents),
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

    fn sample_diff() -> BridgeRequest {
        BridgeRequest::BridgeDiff(BridgeDiff {
            link: Link::new(Ref::bookmark(BookmarkId::new()), Token::from("testtoken")),
            root_hash: Some(ContentHash::new("abc123")),
        })
    }

    fn sample_resolve() -> BridgeRequest {
        BridgeRequest::BridgeResolve(BridgeResolve {
            link: Link::new(Ref::bookmark(BookmarkId::new()), Token::from("testtoken")),
            hashes: vec![ContentHash::new("hash1"), ContentHash::new("hash2")],
        })
    }

    fn sample_fetch() -> BridgeRequest {
        BridgeRequest::BridgeFetchEvents(BridgeFetchEvents {
            link: Link::new(Ref::bookmark(BookmarkId::new()), Token::from("testtoken")),
            event_ids: vec!["id1".into(), "id2".into()],
        })
    }

    #[test]
    fn diff_request_roundtrip_through_bytes() {
        let original = sample_diff();
        let bytes = original.to_bytes();
        let decoded = BridgeRequest::from_bytes(&bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn resolve_request_roundtrip_through_bytes() {
        let original = sample_resolve();
        let bytes = original.to_bytes();
        let decoded = BridgeRequest::from_bytes(&bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn fetch_request_roundtrip_through_bytes() {
        let original = sample_fetch();
        let bytes = original.to_bytes();
        let decoded = BridgeRequest::from_bytes(&bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn diff_request_roundtrip_through_serde_json() {
        let original = sample_diff();
        let json = serde_json::to_string(&original).unwrap();
        let decoded: BridgeRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn diff_request_variant_matches() {
        let req = sample_diff();
        assert!(matches!(req, BridgeRequest::BridgeDiff(_)));
    }

    #[test]
    fn resolve_request_variant_matches() {
        let req = sample_resolve();
        assert!(matches!(req, BridgeRequest::BridgeResolve(_)));
    }
}
