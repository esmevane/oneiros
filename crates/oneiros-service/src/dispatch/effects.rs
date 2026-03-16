use oneiros_db::*;
use oneiros_model::*;

use crate::*;

/// An event sink. Dispatchers produce events, the sink consumes them.
///
/// What the sink does — persist, broadcast, forward over HTTP, collect
/// into a Vec for testing — is the sink's business. The dispatcher
/// just says what happened.
///
/// The sink already knows where to write and which projections to run.
/// This removes all infrastructure knowledge from domain dispatch logic.
pub trait Effects {
    /// Emit a state-changing event — persists and runs projections.
    fn emit(&self, event: &Events) -> Result<Event, Error>;

    /// Mark an observational event — persists without running projections.
    fn mark(&self, event: &Events) -> Result<Event, Error>;
}

/// Production event sink backed by the service's event log and broadcast channel.
///
/// Constructed with everything it needs — database, projections, source
/// identity, broadcast sender. The dispatcher just says what happened.
#[derive(bon::Builder)]
pub struct ServiceEffects<'a> {
    source: Source,
    sender: &'a tokio::sync::broadcast::Sender<Event>,
    db: &'a Database,
    projections: &'a [&'a [Projection]],
}

impl Effects for ServiceEffects<'_> {
    fn emit(&self, event: &Events) -> Result<Event, Error> {
        let new_event = NewEvent::new(event.clone(), self.source);
        let persisted = self.db.log_event(&new_event, self.projections)?;
        let _ = self.sender.send(persisted.clone());
        Ok(persisted)
    }

    fn mark(&self, event: &Events) -> Result<Event, Error> {
        let new_event = NewEvent::new(event.clone(), self.source);
        let persisted = self.db.log_event(&new_event, &[])?;
        let _ = self.sender.send(persisted.clone());
        Ok(persisted)
    }
}
