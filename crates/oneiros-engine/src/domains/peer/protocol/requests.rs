use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// Add a new peer by supplying its address in base64url-encoded form.
///
/// The address carries the cryptographic identity (endpoint id = ed25519
/// public key) plus reachability info (relay URLs, direct sockets).
/// The service extracts the key from the address and stores the full peer
/// record.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct AddPeer {
    /// The peer's address — a base64url-encoded PeerAddress, as produced
    /// by `bookmark share` or extracted from an `oneiros://` URI.
    #[arg(long)]
    pub address: String,
    /// Optional human-readable label. Defaults to a short hex prefix of
    /// the key (e.g. `peer-a3f2b1`).
    #[arg(long)]
    pub name: Option<String>,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetPeer {
    #[builder(into)]
    pub id: PeerId,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemovePeer {
    #[builder(into)]
    pub id: PeerId,
}

#[derive(Builder, Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListPeers {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub filters: SearchFilters,
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
