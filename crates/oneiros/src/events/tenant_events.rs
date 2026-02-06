use crate::*;

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub(crate) enum TenantEvents {
    TenantCreated(Tenant),
}
