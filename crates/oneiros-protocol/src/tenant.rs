use oneiros_model::Tenant;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TenantEvents {
    TenantCreated(Tenant),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TenantRequests {
    CreateTenant(Tenant),
}
