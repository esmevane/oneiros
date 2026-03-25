use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ConnectionRequest {
    Create {
        from_ref: RefToken,
        to_ref: RefToken,
        nature: NatureName,
    },
    Get {
        id: ConnectionId,
    },
    List {
        entity: Option<RefToken>,
    },
    Remove {
        id: ConnectionId,
    },
}
