use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Ticket {
    #[builder(default, into)]
    pub id: TicketId,
    pub actor_id: ActorId,
    pub brain_name: BrainName,
    pub brain_id: BrainId,
    pub token: Token,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

resource_id!(TicketId);
