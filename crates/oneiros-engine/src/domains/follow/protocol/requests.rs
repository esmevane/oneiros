use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum GetFollow {
        V1 => {
            #[builder(into)] pub key: ResourceKey<FollowId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListFollows {
        V1 => {
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = FollowRequestType, display = "kebab-case")]
pub enum FollowRequest {
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
