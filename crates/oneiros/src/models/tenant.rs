use crate::*;

#[derive(serde::Serialize)]
pub(crate) struct Tenant {
    pub(crate) tenant_id: Id,
    pub(crate) name: Label,
}
