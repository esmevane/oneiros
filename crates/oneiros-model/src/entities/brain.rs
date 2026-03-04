use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BrainStatus {
    Active,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Brain {
    pub id: BrainId,
    pub tenant_id: TenantId,
    pub name: BrainName,
    pub status: BrainStatus,
}

impl Brain {
    pub fn init(tenant_id: TenantId, name: BrainName) -> Self {
        Self {
            id: BrainId::new(),
            tenant_id,
            name,
            status: BrainStatus::Active,
        }
    }
}

domain_id!(BrainId);
domain_name!(BrainName);
