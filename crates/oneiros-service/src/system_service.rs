use oneiros_db::Database;
use oneiros_model::*;
use std::path::Path;
use std::sync::MutexGuard;
use tokio::sync::broadcast;

use crate::dispatch::{SystemDispatch, SystemDispatchResponse};
use crate::{Error, projections};

#[derive(Debug, thiserror::Error)]
pub enum CreateBrainError {
    #[error("Malformed input: {0}")]
    MalformedId(#[from] IdParseError),
}

/// Domain service for system-scoped operations.
///
/// Owns the validate → construct → persist → broadcast pipeline for system-level
/// entities (tenants, actors, brains, tickets). Mirrors BrainService but operates
/// on the system Database behind a Mutex rather than a per-request brain Database.
///
/// Holds the MutexGuard directly — the lock lives as long as the service does.
/// Source is a system invariant resolved at ServiceState construction time.
pub struct SystemService<'a> {
    db: MutexGuard<'a, Database>,
    data_dir: &'a Path,
    event_tx: &'a broadcast::Sender<Event>,
    source: Source,
}

impl<'a> SystemService<'a> {
    pub fn new(
        db: MutexGuard<'a, Database>,
        data_dir: &'a Path,
        event_tx: &'a broadcast::Sender<Event>,
        source: Source,
    ) -> Self {
        Self {
            db,
            data_dir,
            event_tx,
            source,
        }
    }

    /// Dispatch any system-scoped request to the appropriate domain dispatcher.
    pub fn dispatch(
        &self,
        request: impl Into<SystemDispatch>,
    ) -> Result<SystemDispatchResponse, Error> {
        match request.into() {
            SystemDispatch::Actor(r) => Ok(SystemDispatchResponse::Actor(self.dispatch_actor(r)?)),
            SystemDispatch::Brain(r) => Ok(SystemDispatchResponse::Brain(self.dispatch_brain(r)?)),
            SystemDispatch::Tenant(r) => {
                Ok(SystemDispatchResponse::Tenant(self.dispatch_tenant(r)?))
            }
            SystemDispatch::Ticket(r) => {
                Ok(SystemDispatchResponse::Ticket(self.dispatch_ticket(r)?))
            }
        }
    }

    /// Persist a state-changing event (runs SYSTEM projections) then broadcast.
    fn log_and_broadcast(&self, event: &Events) -> Result<Event, Error> {
        let new_event = NewEvent::new(event.clone(), self.source);
        let persisted = self.db.log_event(&new_event, projections::SYSTEM)?;
        let _ = self.event_tx.send(persisted.clone());
        Ok(persisted)
    }

    // ── Brain operations ──────────────────────────────────────────────

    pub fn dispatch_brain(&self, request: BrainRequests) -> Result<BrainResponses, Error> {
        match request {
            BrainRequests::CreateBrain(request) => {
                let tenant_id = self.source.tenant_id;
                let actor_id = self.source.actor_id;

                if self
                    .db
                    .brain_exists(tenant_id.to_string(), request.name.as_str())
                    .is_ok()
                {
                    Err(Conflicts::Brain(request.name.clone()))?;
                }

                let brains_dir = self.data_dir.join("brains");
                oneiros_fs::FileOps.ensure_dir(&brains_dir)?;

                let path = brains_dir.join(format!("{}.db", request.name));

                Database::create_brain_db(&path)?;

                let brain = Brain::init(tenant_id, request.name, path);
                let brain_id = brain.id;

                let event = Events::Brain(BrainEvents::BrainCreated(brain));
                self.log_and_broadcast(&event)?;

                let token = Token::issue(TokenClaims {
                    brain_id,
                    tenant_id,
                    actor_id,
                });

                let ticket = Ticket::init(token.clone(), actor_id);

                let ticket_event = Events::Ticket(TicketEvents::TicketIssued(ticket));
                self.log_and_broadcast(&ticket_event)?;

                Ok(BrainResponses::BrainCreated(BrainInfo {
                    entity: brain_id,
                    token,
                }))
            }
            BrainRequests::GetBrain(request) => {
                let tenant_id = self.source.tenant_id;
                let brain = self
                    .db
                    .get_brain_by_name(tenant_id.to_string(), &request.name)?
                    .ok_or(NotFound::BrainByName(request.name))?;
                Ok(BrainResponses::BrainFound(brain))
            }
            BrainRequests::ListBrains(_) => {
                Ok(BrainResponses::BrainsListed(self.db.list_brains()?))
            }
        }
    }

    // ── Tenant operations ─────────────────────────────────────────────

    pub fn dispatch_tenant(&self, request: TenantRequests) -> Result<TenantResponses, Error> {
        match request {
            TenantRequests::GetTenant(request) => {
                let tenant = self
                    .db
                    .get_tenant_by_name(&request.name)?
                    .ok_or(NotFound::Tenant(request.name))?;
                Ok(TenantResponses::TenantFound(tenant))
            }
            TenantRequests::ListTenants(_) => {
                Ok(TenantResponses::TenantsListed(self.db.list_tenants()?))
            }
        }
    }

    // ── Actor operations ──────────────────────────────────────────────

    pub fn dispatch_actor(&self, request: ActorRequests) -> Result<ActorResponses, Error> {
        match request {
            ActorRequests::GetActor(request) => {
                let actor = self
                    .db
                    .get_actor_by_name(&request.name)?
                    .ok_or(NotFound::Actor(request.name))?;
                Ok(ActorResponses::ActorFound(actor))
            }
            ActorRequests::ListActors(_) => {
                Ok(ActorResponses::ActorsListed(self.db.list_actors()?))
            }
        }
    }

    // ── Ticket operations ─────────────────────────────────────────────

    pub fn dispatch_ticket(&self, request: TicketRequests) -> Result<TicketResponses, Error> {
        match request {
            TicketRequests::ValidateTicket(request) => {
                let valid = self.db.validate_ticket(request.token.as_str())?;
                Ok(TicketResponses::TicketValid(valid))
            }
            TicketRequests::ListTickets(_) => {
                Ok(TicketResponses::TicketsListed(self.db.list_tickets()?))
            }
        }
    }
}
