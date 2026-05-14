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

impl ClientRequest for SetNature {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let SetNature::V1(body) = self;
        client.put(&format!("/natures/{}", body.name), self).await
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

impl ClientRequest for GetNature {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let GetNature::V1(lookup) = self;
        client.get(&format!("/natures/{}", lookup.key)).await
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

impl ClientRequest for RemoveNature {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let RemoveNature::V1(removal) = self;
        client.delete(&format!("/natures/{}", removal.name)).await
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

impl ClientRequest for ListNatures {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListNatures::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/natures?{query}")).await
    }
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
