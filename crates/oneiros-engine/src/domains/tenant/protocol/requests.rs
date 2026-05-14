use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CreateTenant {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: TenantName,
        }
    }
}

impl ClientRequest for CreateTenant {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/tenants", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetTenant {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<TenantId>,
        }
    }
}

impl ClientRequest for GetTenant {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let GetTenant::V1(lookup) = self;
        client.get(&format!("/tenants/{}", lookup.key)).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListTenants {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ClientRequest for ListTenants {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListTenants::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/tenants?{query}")).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TenantRequestType, display = "kebab-case")]
pub(crate) enum TenantRequest {
    CreateTenant(CreateTenant),
    GetTenant(GetTenant),
    ListTenants(ListTenants),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (TenantRequestType::CreateTenant, "create-tenant"),
            (TenantRequestType::GetTenant, "get-tenant"),
            (TenantRequestType::ListTenants, "list-tenants"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
