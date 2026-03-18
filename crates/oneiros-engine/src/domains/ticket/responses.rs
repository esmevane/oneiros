use serde::{Deserialize, Serialize};

use super::model::Ticket;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TicketResponse {
    Created(Ticket),
    Found(Ticket),
    Listed(Vec<Ticket>),
    Validated(Ticket),
}
