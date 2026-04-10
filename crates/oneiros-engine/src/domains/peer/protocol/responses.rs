use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = PeerResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PeerResponse {
    Added(Response<Peer>),
    Found(Response<Peer>),
    Listed(Listed<Response<Peer>>),
    Removed(PeerId),
}
