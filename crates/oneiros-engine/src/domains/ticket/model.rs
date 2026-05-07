use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Ticket {
    #[builder(default, into)]
    pub(crate) id: TicketId,
    pub(crate) actor_id: ActorId,
    pub(crate) brain_name: BrainName,
    pub(crate) brain_id: BrainId,
    /// Target + token bundled. The target is a `Ref` pointing at what this
    /// ticket grants access to; the token is the self-describing bearer
    /// credential presented during auth.
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

impl Indexable<TicketId> for Ticket {
    fn id(&self) -> TicketId {
        self.id
    }
}

pub(crate) type Tickets = EntityIndex<TicketId, Ticket>;

resource_id!(TicketId);
