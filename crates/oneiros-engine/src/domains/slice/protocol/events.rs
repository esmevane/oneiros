use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = SliceEventsType, display = "kebab-case")]
pub(crate) enum SliceEvents {
    SliceCreated(SliceCreated),
    SliceDeleted(SliceDeleted),
    SliceMatched(SliceMatched),
}

versioned! {
    pub(crate) enum SliceCreated {
        V1 => {
            #[serde(flatten)]
            pub(crate) slice: Slice,
        }
    }
}

versioned! {
    pub(crate) enum SliceDeleted {
        V1 => {
            #[builder(into)]
            pub(crate) name: SliceName,
        }
    }
}

versioned! {
    /// Emitted by the slice actor when a new event matches a slice's lens.
    /// Carries the matched event's ID for replayability — replaying this
    /// event against the projection increments `event_count` for the named
    /// slice by one.
    pub(crate) enum SliceMatched {
        V1 => {
            #[builder(into)]
            pub(crate) slice_name: SliceName,
            #[builder(into)]
            pub(crate) matched_event_id: EventId,
        }
    }
}
