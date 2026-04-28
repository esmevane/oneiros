use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = PeerResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PeerResponse {
    Added(PeerAddedResponse),
    Found(PeerFoundResponse),
    Listed(PeersResponse),
    Removed(PeerRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum PeerAddedResponse {
        V1 => {
            #[builder(default)] pub id: PeerId,
            pub key: PeerKey,
            pub address: PeerAddress,
            #[builder(into)] pub name: PeerName,
            pub created_at: Timestamp,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum PeerFoundResponse {
        V1 => {
            #[builder(default)] pub id: PeerId,
            pub key: PeerKey,
            pub address: PeerAddress,
            #[builder(into)] pub name: PeerName,
            pub created_at: Timestamp,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum PeersResponse {
        V1 => {
            pub items: Vec<PeerFoundResponseV1>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum PeerRemovedResponse {
        V1 => {
            pub id: PeerId,
        }
    }
}
