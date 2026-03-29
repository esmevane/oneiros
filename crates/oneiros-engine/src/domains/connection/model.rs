use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Connection {
    #[builder(default)]
    pub id: ConnectionId,
    pub from_ref: Ref,
    pub to_ref: Ref,
    #[builder(into)]
    pub nature: NatureName,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

resource_id!(ConnectionId);
