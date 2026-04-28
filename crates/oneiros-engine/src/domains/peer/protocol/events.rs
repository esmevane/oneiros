use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = PeerEventsType, display = "kebab-case")]
pub enum PeerEvents {
    PeerAdded(PeerAdded),
    PeerUpdated(PeerUpdated),
    PeerRemoved(PeerRemoved),
}

versioned! {
    pub enum PeerAdded {
        V1 => {
            #[builder(default)] pub id: PeerId,
            pub key: PeerKey,
            pub address: PeerAddress,
            #[builder(into)] pub name: PeerName,
            #[builder(default = Timestamp::now())] pub created_at: Timestamp,
        }
    }
}

versioned! {
    pub enum PeerUpdated {
        V1 => {
            #[builder(default)] pub id: PeerId,
            pub key: PeerKey,
            pub address: PeerAddress,
            #[builder(into)] pub name: PeerName,
            #[builder(default = Timestamp::now())] pub created_at: Timestamp,
        }
    }
}

versioned! {
    pub enum PeerRemoved {
        V1 => {
            pub id: PeerId,
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
