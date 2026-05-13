use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Ticket {
    #[builder(default, into)]
    pub(crate) id: TicketId,
    pub(crate) actor_id: ActorId,
    #[serde(alias = "brain_name")]
    pub(crate) project_name: ProjectName,
    #[serde(alias = "brain_id")]
    pub(crate) project_id: ProjectId,
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

impl Ticket {
    /// Check whether this ticket is still valid for use. Returns `Ok(())` if
    /// the ticket is neither revoked, expired, nor exhausted. Returns `Err`
    /// with a human-readable reason otherwise.
    pub(crate) fn check_validity(&self) -> Result<(), &'static str> {
        if self.revoked_at.is_some() {
            return Err("ticket has been revoked");
        }
        if let Some(ref expires_at) = self.expires_at
            && *expires_at <= Timestamp::now()
        {
            return Err("ticket has expired");
        }
        if let Some(max_uses) = self.max_uses
            && self.uses >= max_uses
        {
            return Err("ticket use limit reached");
        }
        Ok(())
    }
}

pub(crate) type Tickets = EntityIndex<TicketId, Ticket>;

resource_id!(TicketId);
