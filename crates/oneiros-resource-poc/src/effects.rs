use oneiros_db::{Database, Projection};
use oneiros_model::*;

/// Lightweight event sink for the POC.
///
/// Mirrors `oneiros_service::Effects` — dispatchers produce events,
/// the sink persists them and runs projections. This version drops
/// broadcast (no SSE subscribers in the POC) to avoid pulling in
/// tokio::sync::broadcast.
pub trait Effects {
    /// Emit a state-changing event — persists and runs projections.
    fn emit(&self, event: &Events) -> Result<Event, EffectsError>;

    /// Mark an observational event — persists without running projections.
    fn mark(&self, event: &Events) -> Result<Event, EffectsError>;
}

#[derive(Debug, thiserror::Error)]
pub enum EffectsError {
    #[error(transparent)]
    Database(#[from] oneiros_db::DatabaseError),
}

/// Production-lite event sink backed by the database's event log.
///
/// No broadcast channel — events are persisted and projections run,
/// but no SSE notification. Suitable for proving the dispatch shape.
pub struct PocEffects<'a> {
    source: Source,
    db: &'a Database,
    projections: &'a [&'a [Projection]],
}

impl<'a> PocEffects<'a> {
    pub fn new(source: Source, db: &'a Database, projections: &'a [&'a [Projection]]) -> Self {
        Self {
            source,
            db,
            projections,
        }
    }
}

impl Effects for PocEffects<'_> {
    fn emit(&self, event: &Events) -> Result<Event, EffectsError> {
        let new_event = NewEvent::new(event.clone(), self.source);
        let persisted = self.db.log_event(&new_event, self.projections)?;
        Ok(persisted)
    }

    fn mark(&self, event: &Events) -> Result<Event, EffectsError> {
        let new_event = NewEvent::new(event.clone(), self.source);
        let persisted = self.db.log_event(&new_event, &[])?;
        Ok(persisted)
    }
}
