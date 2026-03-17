//! Agent actor — domain logic for Agent operations.
//!
//! The Agent actor holds a Db handle (its capability to persist)
//! and handles AgentRequests by querying and mutating through the
//! database actor. Cross-resource validation (Persona FK) is done
//! by querying the same Db actor — which is inter-actor messaging
//! from the Agent actor's perspective.

use oneiros_actor::Actor;
use oneiros_model::*;
use tokio::sync::broadcast;

use crate::database::Db;

/// The Agent actor's error type.
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error(transparent)]
    NotFound(#[from] NotFound),

    #[error("Conflict: {0}")]
    Conflict(#[from] Conflicts),
}

/// The Agent actor — holds capabilities and domain logic.
///
/// Capabilities:
/// - `db`: persist and query through the database actor
/// - `events`: broadcast events to subscribers (SSE, projections, etc.)
/// - `source`: provenance identity for event attribution
pub struct AgentActor {
    db: Db,
    events: broadcast::Sender<Event>,
    source: Source,
}

impl AgentActor {
    pub fn new(db: Db) -> Self {
        // No event bus — standalone mode (for backward compat with existing tests)
        let (events, _) = broadcast::channel(16);
        Self {
            db,
            events,
            source: Source::default(),
        }
    }

    pub fn with_bus(db: Db, events: broadcast::Sender<Event>, source: Source) -> Self {
        Self { db, events, source }
    }

    /// Emit an event: persist through DB actor, broadcast to subscribers.
    async fn emit(&self, event_data: Events) -> Event {
        let new_event = NewEvent::new(event_data, self.source);
        let persisted = self.db.log_event(new_event).await;
        // Broadcast — if no subscribers, that's fine (send returns Err on no receivers)
        let _ = self.events.send(persisted.clone());
        persisted
    }
}

impl Actor for AgentActor {
    type Message = AgentRequests;
    type Response = Result<AgentResponses, AgentError>;

    async fn handle(&mut self, message: AgentRequests) -> Result<AgentResponses, AgentError> {
        match message {
            AgentRequests::CreateAgent(request) => {
                // Cross-resource validation: persona must exist
                // This is an inter-actor query — Agent asks the DB actor
                // (which serves all resources) if the persona exists.
                let persona = self.db.get_persona(&request.persona).await;
                if persona.is_none() {
                    return Err(NotFound::Persona(request.persona).into());
                }

                // Conflict detection
                if self.db.agent_name_exists(&request.name).await {
                    return Err(Conflicts::Agent(request.name).into());
                }

                // Entity construction
                let agent = Agent::init(
                    request.description,
                    request.prompt,
                    request.name,
                    request.persona,
                );

                // Event emission — through the DB actor
                self.emit(Events::Agent(AgentEvents::AgentCreated(agent.clone()))).await;

                Ok(AgentResponses::AgentCreated(agent))
            }

            AgentRequests::ListAgents(_) => {
                let agents = self.db.list_agents().await;
                Ok(AgentResponses::AgentsListed(agents))
            }

            AgentRequests::GetAgent(request) => {
                let agent = self
                    .db
                    .get_agent(&request.name)
                    .await
                    .ok_or(NotFound::Agent(request.name))?;
                Ok(AgentResponses::AgentFound(agent))
            }

            AgentRequests::UpdateAgent(request) => {
                let existing = self
                    .db
                    .get_agent(&request.name)
                    .await
                    .ok_or(NotFound::Agent(request.name.clone()))?;

                let persona = self.db.get_persona(&request.persona).await;
                if persona.is_none() {
                    return Err(NotFound::Persona(request.persona).into());
                }

                let agent = Agent::construct(
                    existing.id,
                    request.description,
                    request.prompt,
                    existing.name.clone(),
                    request.persona,
                );

                self.emit(Events::Agent(AgentEvents::AgentUpdated(agent.clone()))).await;

                Ok(AgentResponses::AgentUpdated(agent))
            }

            AgentRequests::RemoveAgent(request) => {
                self.emit(Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName {
                    name: request.name,
                }))).await;

                Ok(AgentResponses::AgentRemoved)
            }
        }
    }
}
