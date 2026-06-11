use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SetSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: SensationName,
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
    pub(crate) enum GetSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<SensationName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoveSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: SensationName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListSensations {
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
    SetSensation => |this, client| {
        let SetSensation::V1(body) = this;
        client
            .put(&format!("/sensations/{}", body.name), this)
            .await
    },
    GetSensation => |this, client| {
        let GetSensation::V1(lookup) = this;
        client.get(&format!("/sensations/{}", lookup.key)).await
    },
    RemoveSensation => |this, client| {
        let RemoveSensation::V1(removal) = this;
        client
            .delete(&format!("/sensations/{}", removal.name))
            .await
    },
    ListSensations => |this, client| {
        let ListSensations::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/sensations?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SensationRequestType, display = "kebab-case")]
pub(crate) enum SensationRequest {
    SetSensation(SetSensation),
    GetSensation(GetSensation),
    ListSensations(ListSensations),
    RemoveSensation(RemoveSensation),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (SensationRequestType::SetSensation, "set-sensation"),
            (SensationRequestType::GetSensation, "get-sensation"),
            (SensationRequestType::ListSensations, "list-sensations"),
            (SensationRequestType::RemoveSensation, "remove-sensation"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
