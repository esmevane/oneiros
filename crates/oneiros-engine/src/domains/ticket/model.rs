use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Ticket {
    pub id: String,
    pub actor_id: ActorId,
    pub brain_name: String,
    pub token: String,
    pub created_at: String,
}
