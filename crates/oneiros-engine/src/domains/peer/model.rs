use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A known remote host — the persisted identity of a peer this host has
/// learned about.
///
/// `id` is the internal domain handle (UUID), used for references and
/// local bookkeeping. `key` is the cryptographic identity (ed25519 public
/// key) — unforgeable, the thing iroh actually verifies during connection
/// establishment. `address` is the current reachability info (may change
/// as the peer's network environment shifts). `name` is the human-readable
/// label for display, defaulting to a short hex prefix of the key when no
/// explicit name is provided.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Peer {
    #[builder(default)]
    pub(crate) id: PeerId,
    pub(crate) key: PeerKey,
    pub(crate) address: PeerAddress,
    #[builder(into)]
    pub(crate) name: PeerName,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Indexable<PeerId> for Peer {
    fn id(&self) -> PeerId {
        self.id
    }
}

pub(crate) type Peers = EntityIndex<PeerId, Peer>;

resource_id!(PeerId);
resource_name!(PeerName);
