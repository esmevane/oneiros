use crate::*;

pub struct AgentService;

impl AgentService {
    pub fn create(
        ctx: &ProjectContext,
        name: String,
        persona: String,
        description: String,
        prompt: String,
    ) -> Result<AgentResponse, AgentError> {
        // Cross-resource validation: persona must exist
        let persona_exists = ctx
            .with_db(|conn| PersonaRepo::new(conn).get(&persona))
            .map_err(AgentError::Database)?;

        if persona_exists.is_none() {
            return Err(AgentError::PersonaNotFound(persona));
        }

        // Validate name uniqueness
        let already_exists = ctx
            .with_db(|conn| AgentRepo::new(conn).name_exists(&name))
            .map_err(AgentError::Database)?;

        if already_exists {
            return Err(AgentError::Conflict(name));
        }

        let agent = Agent {
            id: AgentId::new(),
            name: AgentName::new(&name),
            persona,
            description,
            prompt,
        };

        ctx.emit(AgentEvents::AgentCreated(agent.clone()));
        Ok(AgentResponse::Created(agent))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<AgentResponse, AgentError> {
        let agent = ctx
            .with_db(|conn| AgentRepo::new(conn).get(name))
            .map_err(AgentError::Database)?
            .ok_or_else(|| AgentError::NotFound(name.to_string()))?;
        Ok(AgentResponse::Found(agent))
    }

    pub fn list(ctx: &ProjectContext) -> Result<AgentResponse, AgentError> {
        let agents = ctx
            .with_db(|conn| AgentRepo::new(conn).list())
            .map_err(AgentError::Database)?;
        Ok(AgentResponse::Listed(agents))
    }

    pub fn update(
        ctx: &ProjectContext,
        name: String,
        persona: String,
        description: String,
        prompt: String,
    ) -> Result<AgentResponse, AgentError> {
        // Fetch the existing agent to carry forward its id.
        let existing = ctx
            .with_db(|conn| AgentRepo::new(conn).get(&name))
            .map_err(AgentError::Database)?
            .ok_or_else(|| AgentError::NotFound(name.clone()))?;

        let agent = Agent {
            id: existing.id,
            name: AgentName::new(&name),
            persona,
            description,
            prompt,
        };

        ctx.emit(AgentEvents::AgentUpdated(agent.clone()));
        Ok(AgentResponse::Updated(agent))
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<AgentResponse, AgentError> {
        // Confirm existence before emitting removal.
        let exists = ctx
            .with_db(|conn| AgentRepo::new(conn).name_exists(name))
            .map_err(AgentError::Database)?;

        if !exists {
            return Err(AgentError::NotFound(name.to_string()));
        }

        ctx.emit(AgentEvents::AgentRemoved(AgentRemoved {
            name: AgentName::new(name),
        }));
        Ok(AgentResponse::Removed)
    }
}
