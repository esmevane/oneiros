use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = FollowResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[expect(
    clippy::large_enum_variant,
    reason = "We can reduce the size of the Follow later"
)]
pub(crate) enum FollowResponse {
    Found(FollowFoundResponse),
    Listed(FollowsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum FollowFoundResponse {
        V1 => {
            #[builder(default)] pub(crate) id: FollowId,
            pub(crate) brain: BrainName,
            pub(crate) bookmark: BookmarkName,
            pub(crate) source: FollowSource,
            pub(crate) checkpoint: Checkpoint,
            pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum FollowsResponse {
        V1 => {
            pub(crate) items: Vec<FollowFoundResponseV1>,
            pub(crate) total: usize,
        }
    }
}
