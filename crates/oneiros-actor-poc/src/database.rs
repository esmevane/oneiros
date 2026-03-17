//! Database actor — owns the !Send Database on a dedicated thread.
//!
//! The actor accepts closures that execute against the Database reference.
//! This avoids a god enum of DB operations while preserving type safety
//! at the call site through the typed Db wrapper.

use oneiros_actor::{Handle, SyncActor, spawn_sync};
use oneiros_db::{Database, Projection};
use oneiros_model::*;

/// A query closure sent to the database actor.
///
/// The closure captures parameters, executes against &Database + projections,
/// and returns a type-erased boxed result. The Db wrapper restores type safety.
type QueryFn = Box<dyn FnOnce(&Database, &[&[Projection]]) -> Box<dyn std::any::Any + Send> + Send>;

/// The state held by the database actor on its dedicated thread.
struct DatabaseState {
    db: Database,
    projections: &'static [&'static [Projection]],
}

impl SyncActor for DatabaseState {
    type Message = QueryFn;
    type Response = Box<dyn std::any::Any + Send>;

    fn handle(&mut self, query: QueryFn) -> Box<dyn std::any::Any + Send> {
        query(&self.db, self.projections)
    }
}

/// A typed handle to the database actor.
///
/// Each method constructs the right closure and downcasts the result.
/// Callers get full type safety; the actor channel is type-erased internally.
#[derive(Clone)]
pub struct Db {
    handle: Handle<QueryFn, Box<dyn std::any::Any + Send>>,
}

impl Db {
    /// Spawn from an existing database (for testing).
    pub fn spawn(db: Database, projections: &'static [&'static [Projection]]) -> Self {
        // Database is !Send — hand it to the actor thread via oneshot.
        let (tx, rx) = tokio::sync::oneshot::channel();

        let handle = spawn_sync(move || {
            let db = rx.blocking_recv().expect("receive database");
            DatabaseState { db, projections }
        });

        tx.send(db).ok().expect("send database to actor thread");

        Self { handle }
    }

    /// Execute a typed query against the database actor.
    async fn query<T: Send + 'static>(
        &self,
        f: impl FnOnce(&Database, &[&[Projection]]) -> T + Send + 'static,
    ) -> T {
        let result = self
            .handle
            .send(Box::new(move |db, proj| {
                Box::new(f(db, proj)) as Box<dyn std::any::Any + Send>
            }))
            .await
            .expect("db actor alive");

        *result.downcast::<T>().expect("type mismatch in db query")
    }

    // ── Typed convenience methods ───────────────────────────────────

    pub async fn get_agent(&self, name: &AgentName) -> Option<Agent> {
        let name = name.clone();
        self.query(move |db, _| db.get_agent(&name).expect("db error"))
            .await
    }

    pub async fn list_agents(&self) -> Vec<Agent> {
        self.query(|db, _| db.list_agents().expect("db error")).await
    }

    pub async fn agent_name_exists(&self, name: &AgentName) -> bool {
        let name = name.clone();
        self.query(move |db, _| db.agent_name_exists(&name).expect("db error"))
            .await
    }

    pub async fn get_persona(&self, name: &PersonaName) -> Option<Persona> {
        let name = name.clone();
        self.query(move |db, _| db.get_persona(&name).expect("db error"))
            .await
    }

    pub async fn log_event(&self, event: NewEvent) -> Event {
        self.query(move |db, projections| db.log_event(&event, projections).expect("db error"))
            .await
    }

    pub async fn set_persona(&self, name: &PersonaName, description: &Description, prompt: &Prompt) {
        let name = name.clone();
        let description = description.clone();
        let prompt = prompt.clone();
        self.query(move |db, _| {
            db.set_persona(&name, &description, &prompt)
                .expect("db error")
        })
        .await
    }
}
