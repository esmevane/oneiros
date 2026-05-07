use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = TicketResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum TicketResponse {
    Created(TicketCreatedResponse),
    Found(TicketFoundResponse),
    Listed(TicketsResponse),
    Validated(TicketValidatedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TicketCreatedResponse {
        V1 => { #[serde(flatten)] pub(crate) ticket: Ticket }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TicketFoundResponse {
        V1 => { #[serde(flatten)] pub(crate) ticket: Ticket }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TicketsResponse {
        V1 => {
            pub(crate) items: Vec<Ticket>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TicketValidatedResponse {
        V1 => { #[serde(flatten)] pub(crate) ticket: Ticket }
    }
}
