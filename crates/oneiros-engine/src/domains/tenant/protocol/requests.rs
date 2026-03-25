use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TenantRequest {
    Create { name: TenantName },
    Get { id: TenantId },
    List,
}
