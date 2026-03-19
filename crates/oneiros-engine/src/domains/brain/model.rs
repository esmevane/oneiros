use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Brain {
    pub name: BrainName,
    pub created_at: String,
}

resource_name!(BrainName);
resource_id!(BrainId);
