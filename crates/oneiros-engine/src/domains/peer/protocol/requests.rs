use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum AddPeer {
        #[derive(clap::Args)]
        V1 => {
            pub address: String,
            #[arg(long)]
            pub name: Option<String>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetPeer {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<PeerId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RemovePeer {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub id: PeerId,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListPeers {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = PeerRequestType, display = "kebab-case")]
pub enum PeerRequest {
    AddPeer(AddPeer),
    GetPeer(GetPeer),
    RemovePeer(RemovePeer),
    ListPeers(ListPeers),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (PeerRequestType::AddPeer, "add-peer"),
            (PeerRequestType::GetPeer, "get-peer"),
            (PeerRequestType::RemovePeer, "remove-peer"),
            (PeerRequestType::ListPeers, "list-peers"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
