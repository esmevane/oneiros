use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Ticket {
    pub id: TicketId,
    pub token: Token,
    pub created_by: ActorId,
}

impl Ticket {
    pub fn init(token: Token, created_by: ActorId) -> Self {
        Self {
            id: TicketId::new(),
            token,
            created_by,
        }
    }
}

domain_id!(TicketId);
