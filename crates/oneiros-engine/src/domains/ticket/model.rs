use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Ticket {
    #[builder(default, into)]
    pub id: TicketId,
    pub actor_id: ActorId,
    pub brain_name: BrainName,
    pub brain_id: BrainId,
    /// Target + token bundled. The target is a `Ref` pointing at what this
    /// ticket grants access to; the token is the self-describing bearer
    /// credential presented during auth.
    pub link: Link,
    /// The actor who issued this ticket. For self-issued tickets (the
    /// current non-distribution case) this matches `actor_id`.
    #[builder(into)]
    pub granted_by: ActorId,
    pub expires_at: Option<Timestamp>,
    pub revoked_at: Option<Timestamp>,
    pub max_uses: Option<u64>,
    #[builder(default)]
    pub uses: u64,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Clone, Default)]
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
