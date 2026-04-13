use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = TicketResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum TicketResponse {
    Created(Ticket),
    Found(Ticket),
    Listed(Listed<Ticket>),
    Validated(Ticket),
}
