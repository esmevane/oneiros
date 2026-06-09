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

    /// Capabilities granted by this ticket. Empty vec = implicit read access
    /// (V0 behavior for backward compatibility).
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) permissions: Vec<Permission>,
}

impl Indexable<TicketId> for Ticket {
    fn id(&self) -> TicketId {
        self.id
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub(crate) enum TicketInvalid {
    #[error("ticket has been revoked")]
    Revoked,
    #[error("ticket has expired")]
    Expired,
    #[error("ticket use limit reached")]
    Exhausted,
}

impl Ticket {
    /// Check whether this ticket is still valid for use. Returns `Ok(())` if
    /// the ticket is neither revoked, expired, nor exhausted.
    pub(crate) fn check_validity(&self) -> Result<(), TicketInvalid> {
        if self.revoked_at.is_some() {
            return Err(TicketInvalid::Revoked);
        }
        if let Some(ref expires_at) = self.expires_at
            && *expires_at <= Timestamp::now()
        {
            return Err(TicketInvalid::Expired);
        }
        if let Some(max_uses) = self.max_uses
            && self.uses >= max_uses
        {
            return Err(TicketInvalid::Exhausted);
        }
        Ok(())
    }

    /// Check whether this ticket grants a specific capability.
    pub(crate) fn can(&self, required: PermissionOp) -> bool {
        if self.permissions.is_empty() {
            return false;
        }
        self.permissions.iter().any(|p| match p.current() {
            Ok(v1) => v1.operation == required,
            Err(_) => false,
        })
    }
}

pub(crate) type Tickets = EntityIndex<TicketId, Ticket>;

resource_id!(TicketId);
