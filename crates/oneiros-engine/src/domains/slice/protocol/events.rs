use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(
    kind = SliceEventsType,
    display = "kebab-case",
    attrs(
        expect(
            clippy::enum_variant_names,
            reason = "We use these for `type` notation in serde"
        )
    )
)]
#[expect(
    clippy::enum_variant_names,
    reason = "We use these for `type` notation in serde"
)]
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
            #[builder(default)]
            pub(crate) initial_event_ids: Vec<EventId>,
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
    /// The projection inserts into `slice_chronicle` (idempotent via
    /// INSERT OR IGNORE). `event_count` is computed from the chronicle.
    pub(crate) enum SliceMatched {
        V1 => {
            #[builder(into)]
            pub(crate) slice_name: SliceName,
            #[builder(into)]
            pub(crate) matched_event_id: EventId,
        }
    }
}
