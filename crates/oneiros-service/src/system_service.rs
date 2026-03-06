use oneiros_db::Database;
use oneiros_model::*;
use std::path::Path;
use std::sync::MutexGuard;
use tokio::sync::broadcast;

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
    event_tx: &'a broadcast::Sender<Events>,
    source: Source,
}

impl<'a> SystemService<'a> {
    pub fn new(
        db: MutexGuard<'a, Database>,
        data_dir: &'a Path,
        event_tx: &'a broadcast::Sender<Events>,
        source: Source,
    ) -> Self {
        Self {
            db,
            data_dir,
            event_tx,
            source,
        }
    }

    /// Persist a state-changing event (runs SYSTEM projections) then broadcast.
    fn log_and_broadcast(&self, event: &Events) -> Result<(), Error> {
        let known = Event::create(event.clone(), self.source);
        self.db.log_event(&known, projections::SYSTEM)?;
        let _ = self.event_tx.send(event.clone());
        Ok(())
    }

    // ── Brain operations ──────────────────────────────────────────────

    pub fn create_brain(&self, request: CreateBrainRequest) -> Result<BrainResponses, Error> {
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

    pub fn list_brains(&self) -> Result<BrainResponses, Error> {
        Ok(BrainResponses::BrainsListed(self.db.list_brains()?))
    }
}
