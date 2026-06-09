use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = RemoteEventsType, display = "kebab-case")]
pub(crate) enum RemoteEvents {
    RemoteAdded(RemoteAdded),
    RemoteRemoved(RemoteRemoved),
}

versioned! {
    pub(crate) enum RemoteAdded {
        V1 => {
            #[serde(flatten)] pub(crate) remote: Remote,
        }
    }
}

versioned! {
    pub(crate) enum RemoteRemoved {
        V1 => {
            pub(crate) id: RemoteId,
        }
    }
}
