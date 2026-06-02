use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = SliceResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum SliceResponse {
    Created(SliceCreatedResponse),
    Slices(Listed<Slice>),
    Diffed(SliceDiffedResponse),
    Deleted(SliceDeletedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SliceCreatedResponse {
        V1 => {
            #[serde(flatten)]
            pub(crate) slice: Slice,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SliceDiffedResponse {
        V1 => {
            /// Event count in source not in target.
            pub(crate) only_in_source: u64,
            /// Event count in target not in source.
            pub(crate) only_in_target: u64,
            /// Events present in both slices.
            pub(crate) in_both: u64,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SliceDeletedResponse {
        V1 => {
            pub(crate) id: SliceId,
            pub(crate) name: SliceName,
        }
    }
}
