use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
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

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "tickets")]
pub struct Tickets(HashMap<String, Ticket>);

impl Tickets {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, ticket: &Ticket) -> Option<Ticket> {
        self.0.insert(ticket.id.to_string(), ticket.clone())
    }

    pub fn remove(&mut self, ticket_id: TicketId) -> Option<Ticket> {
        self.0.remove(&ticket_id.to_string())
    }
}

resource_id!(TicketId);
