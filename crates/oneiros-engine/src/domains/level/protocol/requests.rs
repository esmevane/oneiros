use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SetLevel {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: LevelName,
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
    pub(crate) enum GetLevel {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<LevelName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoveLevel {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: LevelName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListLevels {
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
    SetLevel => |this, client| {
        let SetLevel::V1(body) = this;
        client.put(&format!("/levels/{}", body.name), this).await
    },
    GetLevel => |this, client| {
        let GetLevel::V1(lookup) = this;
        client.get(&format!("/levels/{}", lookup.key)).await
    },
    RemoveLevel => |this, client| {
        let RemoveLevel::V1(removal) = this;
        client.delete(&format!("/levels/{}", removal.name)).await
    },
    ListLevels => |this, client| {
        let ListLevels::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/levels?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = LevelRequestType, display = "kebab-case")]
pub(crate) enum LevelRequest {
    SetLevel(SetLevel),
    GetLevel(GetLevel),
    ListLevels(ListLevels),
    RemoveLevel(RemoveLevel),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (LevelRequestType::SetLevel, "set-level"),
            (LevelRequestType::GetLevel, "get-level"),
            (LevelRequestType::ListLevels, "list-levels"),
            (LevelRequestType::RemoveLevel, "remove-level"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
