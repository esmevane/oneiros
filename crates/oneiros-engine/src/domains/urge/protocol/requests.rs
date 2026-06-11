use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SetUrge {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: UrgeName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetUrge {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<UrgeName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoveUrge {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: UrgeName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListUrges {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

resource_requests! {
    SetUrge => |this, client| {
        let SetUrge::V1(body) = this;
        client.put(&format!("/urges/{}", body.name), this).await
    },
    GetUrge => |this, client| {
        let GetUrge::V1(lookup) = this;
        client.get(&format!("/urges/{}", lookup.key)).await
    },
    RemoveUrge => |this, client| {
        let RemoveUrge::V1(removal) = this;
        client.delete(&format!("/urges/{}", removal.name)).await
    },
    ListUrges => |this, client| {
        let ListUrges::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/urges?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = UrgeRequestType, display = "kebab-case")]
pub(crate) enum UrgeRequest {
    SetUrge(SetUrge),
    GetUrge(GetUrge),
    ListUrges(ListUrges),
    RemoveUrge(RemoveUrge),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (UrgeRequestType::SetUrge, "set-urge"),
            (UrgeRequestType::GetUrge, "get-urge"),
            (UrgeRequestType::ListUrges, "list-urges"),
            (UrgeRequestType::RemoveUrge, "remove-urge"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
