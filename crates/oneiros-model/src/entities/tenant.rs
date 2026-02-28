use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tenant {
    pub id: TenantId,
    pub name: TenantName,
}

impl Tenant {
    pub fn init(name: TenantName) -> Self {
        Self {
            id: TenantId::new(),
            name,
        }
    }
}

domain_id!(TenantId);
domain_name!(TenantName);
