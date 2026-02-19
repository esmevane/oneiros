use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    pub token: Token,
    pub created_by: ActorId,
}

domain_id!(TicketId);
