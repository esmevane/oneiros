use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    pub ticket_id: TicketId,
    pub token: Token,
    pub created_by: ActorId,
}

domain_id!(TicketId);
