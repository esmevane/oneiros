use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SetNature {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: NatureName,
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
    pub(crate) enum GetNature {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<NatureName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoveNature {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: NatureName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListNatures {
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
    SetNature => |this, client| {
        let SetNature::V1(body) = this;
        client.put(&format!("/natures/{}", body.name), this).await
    },
    GetNature => |this, client| {
        let GetNature::V1(lookup) = this;
        client.get(&format!("/natures/{}", lookup.key)).await
    },
    RemoveNature => |this, client| {
        let RemoveNature::V1(removal) = this;
        client.delete(&format!("/natures/{}", removal.name)).await
    },
    ListNatures => |this, client| {
        let ListNatures::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/natures?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = NatureRequestType, display = "kebab-case")]
pub(crate) enum NatureRequest {
    SetNature(SetNature),
    GetNature(GetNature),
    ListNatures(ListNatures),
    RemoveNature(RemoveNature),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (NatureRequestType::SetNature, "set-nature"),
            (NatureRequestType::GetNature, "get-nature"),
            (NatureRequestType::ListNatures, "list-natures"),
            (NatureRequestType::RemoveNature, "remove-nature"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
