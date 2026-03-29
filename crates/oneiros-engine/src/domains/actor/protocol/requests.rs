use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ActorRequest {
    Create {
        tenant_id: TenantId,
        name: ActorName,
    },
    Get {
        id: ActorId,
    },
    List,
}
