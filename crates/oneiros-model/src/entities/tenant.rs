use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Tenant {
    pub tenant_id: TenantId,
    pub name: TenantName,
}

domain_id!(TenantId);
domain_name!(TenantName);
