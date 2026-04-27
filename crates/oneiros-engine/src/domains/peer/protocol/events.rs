use bon::Builder;
use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = PeerEventsType, display = "kebab-case")]
pub enum PeerEvents {
    /// A peer was added to this host's known-peers table.
    PeerAdded(Peer),
    /// A peer's address (or other mutable field) was refreshed.
    PeerUpdated(Peer),
    /// A peer was explicitly removed from this host's known-peers table.
    PeerRemoved(PeerRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PeerRemoved {
    Current(PeerRemovedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct PeerRemovedV1 {
    pub id: PeerId,
}

impl PeerRemoved {
    pub fn build_v1() -> PeerRemovedV1Builder {
        PeerRemovedV1::builder()
    }

    pub fn id(&self) -> PeerId {
        match self {
            Self::Current(v) => v.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&PeerEventsType::PeerAdded.to_string(), "peer-added");
        assert_eq!(&PeerEventsType::PeerUpdated.to_string(), "peer-updated");
        assert_eq!(&PeerEventsType::PeerRemoved.to_string(), "peer-removed");
    }
}
