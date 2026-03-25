use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Brain {
    #[builder(default = BrainId::new())]
    pub id: BrainId,
    #[builder(into)]
    pub name: BrainName,
    #[builder(default = Timestamp::now() )]
    pub created_at: Timestamp,
}

resource_name!(BrainName);
resource_id!(BrainId);
