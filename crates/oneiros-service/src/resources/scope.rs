use oneiros_db::{Database, Projection};
use oneiros_model::*;
use std::sync::MutexGuard;

use crate::*;

pub trait Scope {
    /// Access the database for reads.
    fn db(&self) -> &Database;

    /// Build effects from the scope's ingredients.
    fn effects(&self) -> ServiceEffects<'_>;

    /// The identity that produced this request.
    fn source(&self) -> Source;
}

/// A locked brain context with everything a resource needs to operate.
///
/// Resources pull from the scope — they don't need the service to wire
/// them up. The scope is the common pool.
pub struct BrainScope<'a> {
    db: MutexGuard<'a, Database>,
    source: Source,
    sender: &'a tokio::sync::broadcast::Sender<Event>,
    projections: &'a [&'a [Projection]],
}

impl<'a> BrainScope<'a> {
    pub fn new(
        db: MutexGuard<'a, Database>,
        source: Source,
        sender: &'a tokio::sync::broadcast::Sender<Event>,
        projections: &'a [&'a [Projection]],
    ) -> Self {
        Self {
            db,
            source,
            sender,
            projections,
        }
    }
}

/// A locked system context — the system-scoped counterpart to BrainScope.
///
/// Same shape, different database. System-scoped stores (Actor, Brain,
/// Tenant, Ticket) pull from this scope.
pub struct SystemScope<'a> {
    db: MutexGuard<'a, Database>,
    source: Source,
    sender: &'a tokio::sync::broadcast::Sender<Event>,
    projections: &'a [&'a [Projection]],
}

impl<'a> SystemScope<'a> {
    pub fn new(
        db: MutexGuard<'a, Database>,
        source: Source,
        sender: &'a tokio::sync::broadcast::Sender<Event>,
        projections: &'a [&'a [Projection]],
    ) -> Self {
        Self {
            db,
            source,
            sender,
            projections,
        }
    }
}

impl Scope for SystemScope<'_> {
    fn db(&self) -> &Database {
        &self.db
    }

    fn effects(&self) -> ServiceEffects<'_> {
        ServiceEffects::builder()
            .db(&self.db)
            .projections(self.projections)
            .source(self.source)
            .sender(self.sender)
            .build()
    }

    fn source(&self) -> Source {
        self.source
    }
}

impl Scope for BrainScope<'_> {
    fn db(&self) -> &Database {
        &self.db
    }

    fn effects(&self) -> ServiceEffects<'_> {
        ServiceEffects::builder()
            .db(&self.db)
            .projections(self.projections)
            .source(self.source)
            .sender(self.sender)
            .build()
    }

    fn source(&self) -> Source {
        self.source
    }
}
