use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub token: Token,
    pub created_by: ActorId,
}

domain_id!(TicketId);
