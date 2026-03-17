//! Agent domain — types, service, and command handling.
//!
//! Following the ducks doc: mod.rs owns the domain logic and the
//! application service. The service receives AppContext (the injected
//! port) and coordinates commands.

pub mod cli;
pub mod http;
pub mod mcp;
mod projections;

pub use projections::PROJECTIONS;

use oneiros_model::*;

use crate::ports::AppContext;

/// Agent domain errors.
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error(transparent)]
    NotFound(#[from] NotFound),

    #[error("Conflict: {0}")]
    Conflict(#[from] Conflicts),

    #[error(transparent)]
    Database(#[from] oneiros_db::DatabaseError),
}

/// The Agent application service.
///
/// In the doc's terms, this is the "command handler" — it coordinates
/// between the domain and the event store port. The driving adapters
/// (http.rs, mcp.rs, cli.rs) all call into this.
pub struct AgentService;

impl AgentService {
    pub fn create(
        ctx: &AppContext,
        request: CreateAgentRequest,
    ) -> Result<AgentResponses, AgentError> {
        // Cross-resource validation: persona must exist
        let persona = ctx.with_db(|db| db.get_persona(&request.persona))?;
        if persona.is_none() {
            return Err(NotFound::Persona(request.persona).into());
        }

        // Conflict detection
        let exists = ctx.with_db(|db| db.agent_name_exists(&request.name))?;
        if exists {
            return Err(Conflicts::Agent(request.name).into());
        }

        // Domain logic
        let agent = Agent::init(
            request.description,
            request.prompt,
            request.name,
            request.persona,
        );

        // Emit event (persist + broadcast)
        ctx.emit(Events::Agent(AgentEvents::AgentCreated(agent.clone())));

        Ok(AgentResponses::AgentCreated(agent))
    }

    pub fn list(ctx: &AppContext) -> Result<AgentResponses, AgentError> {
        let agents = ctx.with_db(|db| db.list_agents())?;
        Ok(AgentResponses::AgentsListed(agents))
    }

    pub fn get(
        ctx: &AppContext,
        name: &AgentName,
    ) -> Result<AgentResponses, AgentError> {
        let agent = ctx
            .with_db(|db| db.get_agent(name))?
            .ok_or(NotFound::Agent(name.clone()))?;
        Ok(AgentResponses::AgentFound(agent))
    }

    pub fn update(
        ctx: &AppContext,
        request: UpdateAgentRequest,
    ) -> Result<AgentResponses, AgentError> {
        let existing = ctx
            .with_db(|db| db.get_agent(&request.name))?
            .ok_or(NotFound::Agent(request.name.clone()))?;

        let persona = ctx.with_db(|db| db.get_persona(&request.persona))?;
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

        ctx.emit(Events::Agent(AgentEvents::AgentUpdated(agent.clone())));

        Ok(AgentResponses::AgentUpdated(agent))
    }

    pub fn remove(
        ctx: &AppContext,
        name: AgentName,
    ) -> Result<AgentResponses, AgentError> {
        ctx.emit(Events::Agent(AgentEvents::AgentRemoved(
            SelectAgentByName { name },
        )));
        Ok(AgentResponses::AgentRemoved)
    }
}
