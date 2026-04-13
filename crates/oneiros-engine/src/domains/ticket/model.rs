use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub(crate) struct Ticket {
    #[builder(default, into)]
    pub(crate) id: TicketId,
    pub(crate) actor_id: ActorId,
    pub(crate) brain_name: BrainName,
    pub(crate) brain_id: BrainId,
    /// Target + token bundled. The target is a `Ref` pointing at what this
    /// ticket grants access to; the token is the self-describing bearer
    /// credential presented during auth. `Link` is serialized as JSON
    /// inside the Loro CRDT since it contains a `Ref` which doesn't derive
    /// lorosurgeon traits.
    #[loro(json)]
    pub(crate) link: Link,
    /// The actor who issued this ticket. For self-issued tickets (the
    /// current non-distribution case) this matches `actor_id`.
    #[builder(into)]
    pub(crate) granted_by: ActorId,
    pub(crate) expires_at: Option<Timestamp>,
    pub(crate) revoked_at: Option<Timestamp>,
    pub(crate) max_uses: Option<u64>,
    #[builder(default)]
    pub(crate) uses: u64,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "tickets")]
pub(crate) struct Tickets(HashMap<String, Ticket>);

impl Tickets {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.0.get(&id.to_string())
    }

    pub(crate) fn set(&mut self, ticket: &Ticket) -> Option<Ticket> {
        self.0.insert(ticket.id.to_string(), ticket.clone())
    }

    pub(crate) fn remove(&mut self, ticket_id: TicketId) -> Option<Ticket> {
        self.0.remove(&ticket_id.to_string())
    }
}

resource_id!(TicketId);
