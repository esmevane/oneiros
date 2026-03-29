use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TicketResponse {
    Created(Ticket),
    Found(Ticket),
    Listed(Vec<Ticket>),
    Validated(Ticket),
}
