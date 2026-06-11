use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = PeerResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum PeerResponse {
    Added(PeerAddedResponse),
    Found(PeerFoundResponse),
    Listed(PeersResponse),
    Removed(PeerRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum PeerAddedResponse {
        V1 => {
            #[builder(default)] pub(crate) id: PeerId,
            pub(crate) key: PeerKey,
            pub(crate) address: PeerAddress,
            #[builder(into)] pub(crate) name: PeerName,
            #[builder(default)] pub(crate) kind: PeerKind,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub(crate) ticket: Option<Link>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub(crate) project: Option<ProjectName>,
            pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum PeerFoundResponse {
        V1 => {
            #[builder(default)] pub(crate) id: PeerId,
            pub(crate) key: PeerKey,
            pub(crate) address: PeerAddress,
            #[builder(into)] pub(crate) name: PeerName,
            #[builder(default)] pub(crate) kind: PeerKind,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub(crate) ticket: Option<Link>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub(crate) project: Option<ProjectName>,
            pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum PeersResponse {
        V1 => {
            pub(crate) items: Vec<PeerFoundResponseV1>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum PeerRemovedResponse {
        V1 => {
            pub(crate) id: PeerId,
        }
    }
}
