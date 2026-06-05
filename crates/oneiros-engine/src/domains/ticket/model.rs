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

    /// Capabilities granted by this ticket.
    ///
    /// An empty vec represents implicit read access — the current behavior
    /// for all existing tickets. Explicit permissions use the versioned
    /// [`Permission`] wrapper.
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
    ///
    /// When `permissions` is empty (V0 behavior, all existing tickets),
    /// only `Read` is granted — `Write` requires an explicit V1 permission.
    pub(crate) fn can(&self, required: PermissionOp) -> bool {
        if self.permissions.is_empty() {
            return required == PermissionOp::Read;
        }
        self.permissions.iter().any(|p| match p.current() {
            Ok(v1) => v1.operation == required,
            Err(_) => false,
        })
    }
}

pub(crate) type Tickets = EntityIndex<TicketId, Ticket>;

resource_id!(TicketId);

#[cfg(test)]
mod tests {
    use super::*;

    fn ticket_with_permissions(ops: &[PermissionOp]) -> Ticket {
        let actor_id = ActorId::new();
        let permissions: Vec<Permission> = ops
            .iter()
            .map(|op| Permission::from(PermissionV1 { operation: *op }))
            .collect();
        Ticket::builder()
            .actor_id(actor_id)
            .project_name(ProjectName::new("test"))
            .project_id(ProjectId::new())
            .link(Link::new(
                Ref::project(ProjectId::new()),
                Token::from("tok"),
            ))
            .granted_by(actor_id)
            .permissions(permissions)
            .build()
    }

    #[test]
    fn empty_permissions_grants_read() {
        let ticket = ticket_with_permissions(&[]);
        assert!(ticket.can(PermissionOp::Read));
    }

    #[test]
    fn empty_permissions_denies_write() {
        let ticket = ticket_with_permissions(&[]);
        assert!(!ticket.can(PermissionOp::Write));
    }

    #[test]
    fn explicit_read_grants_read() {
        let ticket = ticket_with_permissions(&[PermissionOp::Read]);
        assert!(ticket.can(PermissionOp::Read));
    }

    #[test]
    fn explicit_read_denies_write() {
        let ticket = ticket_with_permissions(&[PermissionOp::Read]);
        assert!(!ticket.can(PermissionOp::Write));
    }

    #[test]
    fn explicit_write_denies_read() {
        let ticket = ticket_with_permissions(&[PermissionOp::Write]);
        assert!(!ticket.can(PermissionOp::Read));
    }

    #[test]
    fn explicit_write_grants_write() {
        let ticket = ticket_with_permissions(&[PermissionOp::Write]);
        assert!(ticket.can(PermissionOp::Write));
    }

    #[test]
    fn read_write_grants_both() {
        let ticket = ticket_with_permissions(&[PermissionOp::Read, PermissionOp::Write]);
        assert!(ticket.can(PermissionOp::Read));
        assert!(ticket.can(PermissionOp::Write));
    }

    #[test]
    fn v0_permission_in_vec_is_valid() {
        // A V0 entry in the permissions vec upcasts to Read.
        let actor_id = ActorId::new();
        let ticket = Ticket::builder()
            .actor_id(actor_id)
            .project_name(ProjectName::new("test"))
            .project_id(ProjectId::new())
            .link(Link::new(
                Ref::project(ProjectId::new()),
                Token::from("tok"),
            ))
            .granted_by(actor_id)
            .permissions(vec![Permission::V0(PermissionV0 {})])
            .build();
        assert!(ticket.can(PermissionOp::Read));
        assert!(!ticket.can(PermissionOp::Write));
    }
}
