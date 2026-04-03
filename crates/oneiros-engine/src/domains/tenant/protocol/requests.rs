use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateTenant {
    #[builder(into)]
    pub name: TenantName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetTenant {
    #[builder(into)]
    pub id: TenantId,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListTenants {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TenantRequestType, display = "kebab-case")]
pub enum TenantRequest {
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
