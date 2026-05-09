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

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetTenant {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<TenantId>,
        }
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
