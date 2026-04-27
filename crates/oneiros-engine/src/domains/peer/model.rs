use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Peer {
    Current(PeerV1),
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct PeerV1 {
    #[builder(default)]
    pub id: PeerId,
    pub key: PeerKey,
    pub address: PeerAddress,
    #[builder(into)]
    pub name: PeerName,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

impl Peer {
    pub fn build_v1() -> PeerV1Builder {
        PeerV1::builder()
    }

    pub fn id(&self) -> PeerId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn key(&self) -> PeerKey {
        match self {
            Self::Current(v) => v.key,
        }
    }

    pub fn address(&self) -> &PeerAddress {
        match self {
            Self::Current(v) => &v.address,
        }
    }

    pub fn name(&self) -> &PeerName {
        match self {
            Self::Current(v) => &v.name,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }
}

#[derive(Clone, Default)]
pub struct Peers(HashMap<String, Peer>);

impl Peers {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn values(&self) -> impl Iterator<Item = &Peer> {
        self.0.values()
    }

    pub fn get(&self, id: PeerId) -> Option<&Peer> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, peer: &Peer) -> Option<Peer> {
        self.0.insert(peer.id().to_string(), peer.clone())
    }

    pub fn remove(&mut self, peer_id: PeerId) -> Option<Peer> {
        self.0.remove(&peer_id.to_string())
    }
}

resource_id!(PeerId);
resource_name!(PeerName);
