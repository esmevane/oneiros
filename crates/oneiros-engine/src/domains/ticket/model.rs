use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Ticket {
    Current(TicketV1),
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct TicketV1 {
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

impl Ticket {
    pub fn build_v1() -> TicketV1Builder {
        TicketV1::builder()
    }

    pub fn id(&self) -> TicketId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn actor_id(&self) -> ActorId {
        match self {
            Self::Current(v) => v.actor_id,
        }
    }

    pub fn brain_name(&self) -> &BrainName {
        match self {
            Self::Current(v) => &v.brain_name,
        }
    }

    pub fn brain_id(&self) -> BrainId {
        match self {
            Self::Current(v) => v.brain_id,
        }
    }

    pub fn link(&self) -> &Link {
        match self {
            Self::Current(v) => &v.link,
        }
    }

    pub fn granted_by(&self) -> ActorId {
        match self {
            Self::Current(v) => v.granted_by,
        }
    }

    pub fn expires_at(&self) -> Option<Timestamp> {
        match self {
            Self::Current(v) => v.expires_at,
        }
    }

    pub fn revoked_at(&self) -> Option<Timestamp> {
        match self {
            Self::Current(v) => v.revoked_at,
        }
    }

    pub fn max_uses(&self) -> Option<u64> {
        match self {
            Self::Current(v) => v.max_uses,
        }
    }

    pub fn uses(&self) -> u64 {
        match self {
            Self::Current(v) => v.uses,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }
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
        self.0.insert(ticket.id().to_string(), ticket.clone())
    }

    pub fn remove(&mut self, ticket_id: TicketId) -> Option<Ticket> {
        self.0.remove(&ticket_id.to_string())
    }
}

resource_id!(TicketId);
