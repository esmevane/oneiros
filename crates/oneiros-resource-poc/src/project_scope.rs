use oneiros_db::{Database, Projection};
use oneiros_model::*;
use oneiros_resource::Fulfill;

use crate::{Agent, Effects, PocEffects};

#[derive(Debug, thiserror::Error)]
pub enum ProjectScopeError {
    #[error(transparent)]
    NotFound(#[from] NotFound),

    #[error("Conflict: {0}")]
    Conflict(#[from] Conflicts),

    #[error(transparent)]
    Database(#[from] oneiros_db::DatabaseError),

    #[error(transparent)]
    Effects(#[from] crate::EffectsError),
}

/// Project-level infrastructure context.
///
/// Carries a database reference, source identity, and projections.
/// This is the POC equivalent of `BrainScope` — same infrastructure,
/// new name, owns the Fulfill impls directly.
pub struct ProjectScope<'a> {
    db: &'a Database,
    source: Source,
    projections: &'a [&'a [Projection]],
}

impl<'a> ProjectScope<'a> {
    pub fn new(db: &'a Database, source: Source, projections: &'a [&'a [Projection]]) -> Self {
        Self {
            db,
            source,
            projections,
        }
    }

    fn db(&self) -> &Database {
        self.db
    }

    fn effects(&self) -> PocEffects<'_> {
        PocEffects::new(self.source, self.db, self.projections)
    }
}

/// Fulfill<Agent> for ProjectScope — the domain logic for Agent operations.
///
/// This is a direct port of `AgentStore::dispatch` from oneiros-service,
/// with the scope as `&self` instead of `context.scope`.
impl Fulfill<Agent> for ProjectScope<'_> {
    type Error = ProjectScopeError;

    async fn fulfill(&self, request: AgentRequests) -> Result<AgentResponses, Self::Error> {
        match request {
            AgentRequests::CreateAgent(request) => {
                // Lookup validation: persona must exist
                self.db()
                    .get_persona(&request.persona)?
                    .ok_or(NotFound::Persona(request.persona.clone()))?;

                // Conflict detection: name must be unique
                if self.db().agent_name_exists(&request.name)? {
                    return Err(Conflicts::Agent(request.name).into());
                }

                // Entity construction
                let agent = oneiros_model::Agent::init(
                    request.description,
                    request.prompt,
                    request.name,
                    request.persona,
                );

                // Event emission
                let event = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
                self.effects().emit(&event)?;

                Ok(AgentResponses::AgentCreated(agent))
            }
            AgentRequests::ListAgents(_) => {
                Ok(AgentResponses::AgentsListed(self.db().list_agents()?))
            }
            AgentRequests::GetAgent(request) => {
                let agent = self
                    .db()
                    .get_agent(&request.name)?
                    .ok_or(NotFound::Agent(request.name))?;
                Ok(AgentResponses::AgentFound(agent))
            }
            AgentRequests::UpdateAgent(request) => {
                // Lookup validation: existing agent must exist
                let existing = self
                    .db()
                    .get_agent(&request.name)?
                    .ok_or(NotFound::Agent(request.name.clone()))?;

                // Lookup validation: persona must exist
                self.db()
                    .get_persona(&request.persona)?
                    .ok_or(NotFound::Persona(request.persona.clone()))?;

                // Entity reconstruction
                let agent = oneiros_model::Agent::construct(
                    existing.id,
                    request.description,
                    request.prompt,
                    existing.name.clone(),
                    request.persona,
                );

                let event = Events::Agent(AgentEvents::AgentUpdated(agent.clone()));
                self.effects().emit(&event)?;

                Ok(AgentResponses::AgentUpdated(agent))
            }
            AgentRequests::RemoveAgent(request) => {
                let event = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName {
                    name: request.name,
                }));
                self.effects().emit(&event)?;

                Ok(AgentResponses::AgentRemoved)
            }
        }
    }
}
