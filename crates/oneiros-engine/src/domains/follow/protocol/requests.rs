use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GetFollow {
    #[builder(into)]
    pub(crate) id: FollowId,
}

#[derive(Builder, Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ListFollows {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = FollowRequestType, display = "kebab-case")]
pub(crate) enum FollowRequest {
    GetFollow(GetFollow),
    ListFollows(ListFollows),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        assert_eq!(&FollowRequestType::GetFollow.to_string(), "get-follow");
        assert_eq!(&FollowRequestType::ListFollows.to_string(), "list-follows");
    }
}
