use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetFollow {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<FollowId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListFollows {
        #[derive(clap::Args)]
        V1 => {
            #[serde(flatten)]
            #[clap(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

resource_requests! {
    GetFollow => |this, client| {
        let GetFollow::V1(lookup) = this;
        client.get(&format!("/follows/{}", lookup.key)).await
    },
    ListFollows => |this, client| {
        let ListFollows::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/follows?{query}")).await
    },
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
