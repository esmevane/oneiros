use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = PeerEventsType, display = "kebab-case")]
pub(crate) enum PeerEvents {
    PeerAdded(PeerAdded),
    PeerUpdated(PeerUpdated),
    PeerRemoved(PeerRemoved),
}

versioned! {
    pub(crate) enum PeerAdded {
        V1 => {
            #[builder(default)] pub(crate) id: PeerId,
            pub(crate) key: PeerKey,
            pub(crate) address: PeerAddress,
            #[builder(into)] pub(crate) name: PeerName,
            #[builder(default = Timestamp::now())] pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    pub(crate) enum PeerUpdated {
        V1 => {
            #[builder(default)] pub(crate) id: PeerId,
            pub(crate) key: PeerKey,
            pub(crate) address: PeerAddress,
            #[builder(into)] pub(crate) name: PeerName,
            #[builder(default = Timestamp::now())] pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    pub(crate) enum PeerRemoved {
        V1 => {
            pub(crate) id: PeerId,
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
