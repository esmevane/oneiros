use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = SliceEventsType, display = "kebab-case")]
pub(crate) enum SliceEvents {
    SliceCreated(SliceCreated),
    SliceDeleted(SliceDeleted),
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
