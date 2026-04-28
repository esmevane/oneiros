use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = TicketResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TicketResponse {
    Created(TicketCreatedResponse),
    Found(TicketFoundResponse),
    Listed(TicketsResponse),
    Validated(TicketValidatedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TicketCreatedResponse {
        V1 => { #[serde(flatten)] pub ticket: Ticket }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TicketFoundResponse {
        V1 => { #[serde(flatten)] pub ticket: Ticket }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TicketsResponse {
        V1 => {
            pub items: Vec<Ticket>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TicketValidatedResponse {
        V1 => { #[serde(flatten)] pub ticket: Ticket }
    }
}
