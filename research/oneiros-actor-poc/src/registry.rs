//! Registry — the supervisor that holds actor handles, routes messages,
//! and provides the event bus for broadcast.
//!
//! The registry is the "center" that the trait spike was missing.
//! It knows about all actors and provides typed access to their handles.

use oneiros_actor::Handle;
use oneiros_model::*;
use tokio::sync::broadcast;

use crate::agent::{AgentActor, AgentError};
use crate::database::Db;

/// The application registry — holds actor handles and the event bus.
///
/// For the POC, the registry has named fields for each actor. At scale,
/// this would use a TypeMap or code generation. The named-field approach
/// makes the spike readable and type-safe.
#[derive(Clone)]
pub struct Registry {
    pub agents: Handle<AgentRequests, Result<AgentResponses, AgentError>>,
    pub db: Db,
    pub events: broadcast::Sender<Event>,
}

impl Registry {
    /// Build a registry from a database actor.
    ///
    /// Spawns all resource actors, wires them to the DB, and creates
    /// the event bus.
    pub fn build(db: Db) -> Self {
        Self::build_with_source(db, Source::default())
    }

    pub fn build_with_source(db: Db, source: Source) -> Self {
        let (events_tx, _) = broadcast::channel::<Event>(256);

        let agents =
            oneiros_actor::spawn(AgentActor::with_bus(db.clone(), events_tx.clone(), source));

        Self {
            agents,
            db,
            events: events_tx,
        }
    }

    /// Subscribe to the event bus — returns a receiver for broadcast events.
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.events.subscribe()
    }
}
