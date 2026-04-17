use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use crate::*;

/// The ALPN string advertised and required by the oneiros sync protocol.
/// Only clients explicitly negotiating this ALPN can reach the sync handler.
pub const SYNC_ALPN: &[u8] = b"/oneiros/sync/1";

/// Maximum message size on the sync wire, in bytes. Guards against
/// absurdly large payloads.
pub(crate) const MAX_MESSAGE_SIZE: usize = 64 * 1024 * 1024;

/// The result of a Merkle diff — the server's chronicle root hash
/// and the event IDs the client is missing.
pub struct DiffResult {
    pub server_root: Option<ContentHash>,
    pub missing: Vec<EventId>,
}

/// The bridge to other oneiros hosts — the runtime value that owns the bound
/// `iroh::Endpoint` and acts as our wrapper around the transport layer.
///
/// A Bridge is bound once at service start using the host's persisted
/// keypair, lives on `ServerState`, and produces connections on demand
/// when the system needs to talk to a peer.
#[derive(Clone)]
pub struct Bridge {
    endpoint: iroh::Endpoint,
    public_key: PeerKey,
    router: Arc<OnceLock<iroh::protocol::Router>>,
}

impl Bridge {
    /// Bind a Bridge using the given iroh secret key. Advertises the
    /// `/oneiros/sync/1` ALPN so peers can negotiate the sync protocol.
    pub async fn bind(secret: iroh::SecretKey) -> Result<Self, BridgeError> {
        let public = secret.public();
        let endpoint = iroh::Endpoint::empty_builder()
            .secret_key(secret)
            .alpns(vec![SYNC_ALPN.to_vec()])
            .bind()
            .await
            .map_err(|e| BridgeError::from(IrohError::from(e)))?;

        let public_key = PeerKey::from_bytes(*public.as_bytes());

        Ok(Self {
            endpoint,
            public_key,
            router: Arc::new(OnceLock::new()),
        })
    }

    /// Register the sync protocol handler on this bridge's endpoint.
    /// Idempotent: calling it multiple times has no effect after the first.
    pub fn serve(&self, config: Config, canons: CanonIndex) {
        if self.router.get().is_some() {
            return;
        }
        let handler = SyncHandler::new(config, canons);
        let router = iroh::protocol::Router::builder(self.endpoint.clone())
            .accept(SYNC_ALPN, handler)
            .spawn();
        let _ = self.router.set(router);
    }

    /// Run the Merkle diff protocol against a peer. Walks the peer's
    /// chronicle tree level by level, comparing against the local
    /// chronicle to identify missing event IDs.
    ///
    /// Returns the server's root hash (for checkpoint storage) and
    /// the event IDs the local side is missing.
    pub async fn diff(
        &self,
        address: &PeerAddress,
        link: &Link,
        local_root: Option<&ContentHash>,
        local_resolve: &(impl Fn(&ContentHash) -> Option<LedgerNode> + Send + Sync),
    ) -> Result<DiffResult, BridgeError> {
        // Round 1: send our root hash, get server's root node.
        let diff_request = BridgeRequest::BridgeDiff(BridgeDiff {
            link: link.clone(),
            root_hash: local_root.cloned(),
        });
        let response = self.send(address, &diff_request).await?;

        match response {
            BridgeResponse::BridgeCurrent => Ok(DiffResult {
                server_root: local_root.cloned(),
                missing: vec![],
            }),
            BridgeResponse::BridgeRootNode(root_node) => {
                let server_root = root_node.root_hash;

                // Cache of resolved remote nodes.
                let mut remote: HashMap<ContentHash, LedgerNode> = HashMap::new();
                remote.insert(server_root.clone(), root_node.node);

                // Walk the tree, requesting unresolved nodes in batches.
                loop {
                    let needed =
                        collect_unresolved(local_root, &server_root, local_resolve, &remote);

                    if needed.is_empty() {
                        break;
                    }

                    let resolve_request = BridgeRequest::BridgeResolve(BridgeResolve {
                        link: link.clone(),
                        hashes: needed,
                    });
                    let response = self.send(address, &resolve_request).await?;

                    match response {
                        BridgeResponse::BridgeNodes(bn) => {
                            for (hash, node) in bn.nodes {
                                remote.insert(hash, node);
                            }
                        }
                        BridgeResponse::BridgeDenied(d) => {
                            return Err(BridgeError::Denied(d.reason));
                        }
                        _ => {
                            return Err(BridgeError::Protocol(
                                "expected bridge-nodes response for resolve request".into(),
                            ));
                        }
                    }
                }

                // All server nodes are cached. Run the full diff locally.
                let combined_resolve =
                    |hash: &ContentHash| remote.get(hash).cloned().or_else(|| local_resolve(hash));

                let changes = Ledger::diff(local_root, Some(&server_root), &combined_resolve);

                let missing: Vec<EventId> = changes
                    .into_iter()
                    .filter_map(|c| match c {
                        LedgerChange::Added(id) => Some(id),
                        LedgerChange::Removed(_) => None,
                    })
                    .collect();

                Ok(DiffResult {
                    server_root: Some(server_root),
                    missing,
                })
            }
            BridgeResponse::BridgeDenied(d) => Err(BridgeError::Denied(d.reason)),
            _ => Err(BridgeError::Protocol(
                "expected bridge-current or bridge-root-node response for diff request".into(),
            )),
        }
    }

    /// Fetch specific events by ID from a peer. Issued after the Merkle
    /// diff has identified which events the local side is missing.
    pub async fn fetch_events(
        &self,
        address: &PeerAddress,
        request: &BridgeRequest,
    ) -> Result<Vec<StoredEvent>, BridgeError> {
        let response = self.send(address, request).await?;

        match response {
            BridgeResponse::BridgeEvents(be) => Ok(be.events),
            BridgeResponse::BridgeCurrent => Ok(Vec::new()),
            BridgeResponse::BridgeDenied(d) => Err(BridgeError::Denied(d.reason)),
            _ => Err(BridgeError::Protocol(
                "expected bridge-events response for fetch-events request".into(),
            )),
        }
    }

    /// The host's public key — stable across restarts if the caller binds
    /// with the same secret key.
    pub fn key(&self) -> PeerKey {
        self.public_key
    }

    /// The host's current reachability information.
    pub fn address(&self) -> PeerAddress {
        let mut addr = iroh::EndpointAddr::new(self.endpoint.id());
        for socket in self.endpoint.bound_sockets() {
            addr = addr.with_ip_addr(socket);
        }
        PeerAddress::new(addr)
    }

    /// Compose the host's current identity (key + address).
    pub fn host_identity(&self) -> HostIdentity {
        HostIdentity::new(self.key(), self.address())
    }

    /// Shut down the bridge gracefully, closing the iroh endpoint.
    pub async fn shutdown(&self) {
        self.endpoint.close().await;
    }

    /// Send a sync request to a peer and read the response.
    async fn send(
        &self,
        address: &PeerAddress,
        request: &BridgeRequest,
    ) -> Result<BridgeResponse, BridgeError> {
        let (response_bytes, conn) = self.exchange(address, request).await?;
        conn.close(0u32.into(), b"done");

        if response_bytes.len() > MAX_MESSAGE_SIZE {
            return Err(BridgeError::Protocol(format!(
                "response too large: {} bytes",
                response_bytes.len()
            )));
        }

        BridgeResponse::from_bytes(&response_bytes)
            .map_err(|e| BridgeError::Protocol(e.to_string()))
    }

    /// Low-level transport: connect, write request, read response bytes.
    /// Returns the raw response bytes and the connection (for caller to close).
    async fn exchange(
        &self,
        address: &PeerAddress,
        request: &BridgeRequest,
    ) -> Result<(Vec<u8>, iroh::endpoint::Connection), IrohError> {
        let conn = self
            .endpoint
            .connect(address.inner().clone(), SYNC_ALPN)
            .await?;

        let (mut send, mut recv) = conn.open_bi().await?;

        // Write length-prefixed request.
        let encoded = request.to_bytes();
        let len = (encoded.len() as u32).to_be_bytes();
        send.write_all(&len).await?;
        send.write_all(&encoded).await?;
        send.finish()?;

        // Read length-prefixed response.
        let mut len_buf = [0u8; 4];
        recv.read_exact(&mut len_buf).await?;
        let response_len = u32::from_be_bytes(len_buf) as usize;

        let mut buf = vec![0u8; response_len];
        recv.read_exact(&mut buf).await?;

        Ok((buf, conn))
    }
}

/// Walk the server's HAMT tree as far as cached, collecting hashes of
/// server nodes we haven't resolved yet and that differ from our local tree.
///
/// Returns an empty vec when all reachable server nodes are cached,
/// meaning the diff can be computed locally.
fn collect_unresolved(
    local_root: Option<&ContentHash>,
    server_root: &ContentHash,
    local_resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
    remote: &HashMap<ContentHash, LedgerNode>,
) -> Vec<ContentHash> {
    let mut needed = Vec::new();

    // Stack: (local_hash, server_hash)
    let mut stack: Vec<(Option<ContentHash>, ContentHash)> =
        vec![(local_root.cloned(), server_root.clone())];

    while let Some((local_hash, server_hash)) = stack.pop() {
        // Same hash → same subtree → no diff in this branch.
        if local_hash.as_ref() == Some(&server_hash) {
            continue;
        }

        // Try to get the server node from cache.
        let Some(server_node) = remote.get(&server_hash) else {
            needed.push(server_hash);
            continue;
        };

        match server_node {
            LedgerNode::Leaf { .. } => {
                // Leaf is cached — Ledger::diff will handle the entry-level comparison.
            }
            LedgerNode::Interior {
                children: server_children,
            } => {
                // Resolve the local node to compare children.
                let local_children = local_hash.as_ref().and_then(|h| {
                    local_resolve(h).and_then(|n| match n {
                        LedgerNode::Interior { children } => Some(children),
                        _ => None,
                    })
                });

                for (nibble, server_child_hash) in server_children {
                    let local_child = local_children.as_ref().and_then(|c| c.get(nibble)).cloned();

                    if local_child.as_ref() == Some(server_child_hash) {
                        continue;
                    }

                    stack.push((local_child, server_child_hash.clone()));
                }
            }
        }
    }

    needed
}

impl core::fmt::Debug for Bridge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Bridge")
            .field("public_key", &self.public_key.to_string())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bridge_binds_with_fresh_key() {
        let secret = iroh::SecretKey::generate(&mut rand::rng());
        let expected_key = PeerKey::from_bytes(*secret.public().as_bytes());

        let bridge = Bridge::bind(secret).await.unwrap();
        assert_eq!(bridge.key(), expected_key);

        bridge.shutdown().await;
    }

    #[tokio::test]
    async fn bridge_exposes_host_identity() {
        let secret = iroh::SecretKey::generate(&mut rand::rng());
        let bridge = Bridge::bind(secret).await.unwrap();

        let identity = bridge.host_identity();
        assert_eq!(identity.key, bridge.key());

        bridge.shutdown().await;
    }

    #[tokio::test]
    async fn two_bridges_have_distinct_keys() {
        let a = Bridge::bind(iroh::SecretKey::generate(&mut rand::rng()))
            .await
            .unwrap();
        let b = Bridge::bind(iroh::SecretKey::generate(&mut rand::rng()))
            .await
            .unwrap();

        assert_ne!(a.key(), b.key());

        a.shutdown().await;
        b.shutdown().await;
    }

    #[test]
    fn collect_unresolved_with_matching_roots() {
        let root = ContentHash::new("same_root");
        let needed = collect_unresolved(Some(&root), &root, &|_| None, &HashMap::new());
        assert!(needed.is_empty(), "matching roots should need nothing");
    }

    #[test]
    fn collect_unresolved_with_uncached_root() {
        let local = ContentHash::new("local_root");
        let server = ContentHash::new("server_root");
        let needed = collect_unresolved(Some(&local), &server, &|_| None, &HashMap::new());
        assert_eq!(needed.len(), 1);
        assert_eq!(needed[0], server);
    }

    #[test]
    fn collect_unresolved_with_cached_leaf() {
        use std::collections::BTreeMap;

        let local = ContentHash::new("local_root");
        let server = ContentHash::new("server_root");

        let mut remote = HashMap::new();
        remote.insert(
            server.clone(),
            LedgerNode::Leaf {
                entries: BTreeMap::from([("evt1".into(), ContentHash::new("h1"))]),
            },
        );

        let needed = collect_unresolved(Some(&local), &server, &|_| None, &remote);
        assert!(
            needed.is_empty(),
            "cached leaf should not need further resolution"
        );
    }

    #[test]
    fn collect_unresolved_with_interior_skips_matching_children() {
        use std::collections::BTreeMap;

        let shared_child = ContentHash::new("shared");
        let server_only = ContentHash::new("server_only");

        let local_root = ContentHash::new("local_root");
        let server_root = ContentHash::new("server_root");

        let local_interior = LedgerNode::Interior {
            children: BTreeMap::from([(0, shared_child.clone()), (1, ContentHash::new("local_1"))]),
        };

        let server_interior = LedgerNode::Interior {
            children: BTreeMap::from([(0, shared_child.clone()), (1, server_only.clone())]),
        };

        let mut remote = HashMap::new();
        remote.insert(server_root.clone(), server_interior);

        let local_resolve = {
            let local_root = local_root.clone();
            let local_interior = local_interior.clone();
            move |hash: &ContentHash| {
                if *hash == local_root {
                    Some(local_interior.clone())
                } else {
                    None
                }
            }
        };

        let needed = collect_unresolved(Some(&local_root), &server_root, &local_resolve, &remote);
        assert_eq!(needed.len(), 1, "only the differing child should be needed");
        assert_eq!(needed[0], server_only);
    }
}
