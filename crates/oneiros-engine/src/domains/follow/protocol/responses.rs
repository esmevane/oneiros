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
pub enum FollowResponse {
    Found(FollowFoundResponse),
    Listed(FollowsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum FollowFoundResponse {
        V1 => {
            #[builder(default)] pub id: FollowId,
            pub brain: BrainName,
            pub bookmark: BookmarkName,
            pub source: FollowSource,
            pub checkpoint: Checkpoint,
            pub created_at: Timestamp,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum FollowsResponse {
        V1 => {
            pub items: Vec<FollowFoundResponseV1>,
            pub total: usize,
        }
    }
}
