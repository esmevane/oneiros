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

impl ClientRequest for SetUrge {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let SetUrge::V1(body) = self;
        client.put(&format!("/urges/{}", body.name), self).await
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

impl ClientRequest for GetUrge {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let GetUrge::V1(lookup) = self;
        client.get(&format!("/urges/{}", lookup.key)).await
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

impl ClientRequest for RemoveUrge {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let RemoveUrge::V1(removal) = self;
        client.delete(&format!("/urges/{}", removal.name)).await
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

impl ClientRequest for ListUrges {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListUrges::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/urges?{query}")).await
    }
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
