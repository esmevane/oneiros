//! Agent actor — domain logic for Agent operations.
//!
//! The Agent actor holds a Db handle (its capability to persist)
//! and handles AgentRequests by querying and mutating through the
//! database actor. Cross-resource validation (Persona FK) is done
//! by querying the same Db actor — which is inter-actor messaging
//! from the Agent actor's perspective.

use oneiros_actor::Actor;
use oneiros_model::*;

use crate::database::Db;

/// The Agent actor's error type.
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error(transparent)]
    NotFound(#[from] NotFound),

    #[error("Conflict: {0}")]
    Conflict(#[from] Conflicts),
}

/// The Agent actor — holds capabilities (Db handle) and domain logic.
pub struct AgentActor {
    db: Db,
}

impl AgentActor {
    pub fn new(db: Db) -> Self {
        Self { db }
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
                let event = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
                let new_event = NewEvent::new(event, Source::default());
                self.db.log_event(new_event).await;

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

                let event = Events::Agent(AgentEvents::AgentUpdated(agent.clone()));
                let new_event = NewEvent::new(event, Source::default());
                self.db.log_event(new_event).await;

                Ok(AgentResponses::AgentUpdated(agent))
            }

            AgentRequests::RemoveAgent(request) => {
                let event = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName {
                    name: request.name,
                }));
                let new_event = NewEvent::new(event, Source::default());
                self.db.log_event(new_event).await;

                Ok(AgentResponses::AgentRemoved)
            }
        }
    }
}
