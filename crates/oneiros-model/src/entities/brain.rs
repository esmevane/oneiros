use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BrainStatus {
    Active,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Brain {
    pub brain_id: BrainId,
    pub tenant_id: TenantId,
    pub name: BrainName,
    pub path: PathBuf,
    pub status: BrainStatus,
}

domain_id!(BrainId);
domain_name!(BrainName);
