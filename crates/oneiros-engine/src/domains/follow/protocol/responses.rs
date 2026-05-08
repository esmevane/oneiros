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
            #[serde(flatten)] pub(crate) follow: Follow,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum FollowsResponse {
        V1 => {
            #[serde(flatten)] pub(crate) follows: Listed<Response<FollowFoundResponse>>,
        }
    }
}
