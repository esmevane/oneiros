use serde::{Deserialize, Serialize};

use crate::*;

/// The server's root node — sent when chronicle roots differ.
///
/// Contains the server's current chronicle root hash and the
/// HAMT node at that root. The client uses this to begin the
/// Merkle tree walk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeRootNode {
    pub root_hash: ContentHash,
    pub node: LedgerNode,
}

/// Resolved HAMT nodes — the server's response to a resolve request.
///
/// Each entry is a (hash, node) pair. The client caches these
/// and continues walking the tree until all leaves are reached.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeNodes {
    pub nodes: Vec<(ContentHash, LedgerNode)>,
}

/// The requested events, fetched by ID after a Merkle diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeEvents {
    pub events: Vec<StoredEvent>,
}

/// A denial — the server rejected the request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeDenied {
    pub reason: String,
}

/// The response to a [`BridgeRequest`] over the oneiros sync protocol.
///
/// Carried over the `/oneiros/sync/1` ALPN via iroh's QUIC transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BridgeResponse {
    /// The requestor is already up to date — no diff needed.
    BridgeCurrent,

    /// The server's chronicle root node — roots differ, walk begins.
    BridgeRootNode(BridgeRootNode),

    /// Resolved HAMT nodes requested during the tree walk.
    BridgeNodes(BridgeNodes),

    /// The requested events, fetched by ID after the diff.
    BridgeEvents(BridgeEvents),

    /// The server rejected the request.
    BridgeDenied(BridgeDenied),
}

impl BridgeResponse {
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
        matches!(self, Self::BridgeDenied(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn current_response_roundtrip() {
        let original = BridgeResponse::BridgeCurrent;
        let bytes = original.to_bytes();
        let decoded = BridgeResponse::from_bytes(&bytes).unwrap();
        assert!(matches!(decoded, BridgeResponse::BridgeCurrent));
    }

    #[test]
    fn root_node_response_roundtrip() {
        let original = BridgeResponse::BridgeRootNode(BridgeRootNode {
            root_hash: ContentHash::new("abc123"),
            node: LedgerNode::Leaf {
                entries: BTreeMap::from([("evt1".into(), ContentHash::new("h1"))]),
            },
        });
        let bytes = original.to_bytes();
        let decoded = BridgeResponse::from_bytes(&bytes).unwrap();
        match decoded {
            BridgeResponse::BridgeRootNode(rn) => {
                assert_eq!(rn.root_hash, ContentHash::new("abc123"));
            }
            _ => panic!("expected BridgeRootNode variant"),
        }
    }

    #[test]
    fn nodes_response_roundtrip() {
        let original = BridgeResponse::BridgeNodes(BridgeNodes {
            nodes: vec![(
                ContentHash::new("h1"),
                LedgerNode::Leaf {
                    entries: BTreeMap::new(),
                },
            )],
        });
        let bytes = original.to_bytes();
        let decoded = BridgeResponse::from_bytes(&bytes).unwrap();
        match decoded {
            BridgeResponse::BridgeNodes(bn) => assert_eq!(bn.nodes.len(), 1),
            _ => panic!("expected BridgeNodes variant"),
        }
    }

    #[test]
    fn denied_response_roundtrip() {
        let original = BridgeResponse::BridgeDenied(BridgeDenied {
            reason: "ticket expired".into(),
        });
        let bytes = original.to_bytes();
        let decoded = BridgeResponse::from_bytes(&bytes).unwrap();
        assert!(decoded.is_denied());
        match decoded {
            BridgeResponse::BridgeDenied(d) => assert_eq!(d.reason, "ticket expired"),
            _ => panic!("expected BridgeDenied variant"),
        }
    }

    #[test]
    fn current_is_not_denied() {
        assert!(!BridgeResponse::BridgeCurrent.is_denied());
    }
}
