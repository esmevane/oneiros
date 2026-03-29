use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub actor_id: ActorId,
    pub brain_name: BrainName,
    pub brain_id: BrainId,
    pub token: Token,
    pub created_at: Timestamp,
}

resource_id!(TicketId);
