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

        // Normalize name: append .persona if not already present
        let normalized_name = normalize_agent_name(&name, &persona);

        // Validate name uniqueness
        let already_exists = ctx
            .with_db(|conn| AgentRepo::new(conn).name_exists(&normalized_name))
            .map_err(AgentError::Database)?;

        if already_exists {
            return Err(AgentError::Conflict(normalized_name));
        }

        let agent_name = AgentName::new(&normalized_name);

        let agent = Agent {
            id: AgentId::new(),
            name: agent_name.clone(),
            persona,
            description,
            prompt,
        };

        ctx.emit(AgentEvents::AgentCreated(agent));
        Ok(AgentResponse::AgentCreated(agent_name))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<AgentResponse, AgentError> {
        let agent = ctx
            .with_db(|conn| AgentRepo::new(conn).get(name))
            .map_err(AgentError::Database)?
            .ok_or_else(|| AgentError::NotFound(name.to_string()))?;
        Ok(AgentResponse::AgentDetails(agent))
    }

    pub fn list(ctx: &ProjectContext) -> Result<AgentResponse, AgentError> {
        let agents = ctx
            .with_db(|conn| AgentRepo::new(conn).list())
            .map_err(AgentError::Database)?;
        if agents.is_empty() {
            Ok(AgentResponse::NoAgents)
        } else {
            Ok(AgentResponse::Agents(agents))
        }
    }

    pub fn update(
        ctx: &ProjectContext,
        name: String,
        persona: String,
        description: String,
        prompt: String,
    ) -> Result<AgentResponse, AgentError> {
        let existing = ctx
            .with_db(|conn| AgentRepo::new(conn).get(&name))
            .map_err(AgentError::Database)?
            .ok_or_else(|| AgentError::NotFound(name.clone()))?;

        let agent_name = AgentName::new(&name);

        let agent = Agent {
            id: existing.id,
            name: agent_name.clone(),
            persona,
            description,
            prompt,
        };

        ctx.emit(AgentEvents::AgentUpdated(agent));
        Ok(AgentResponse::AgentUpdated(agent_name))
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<AgentResponse, AgentError> {
        let exists = ctx
            .with_db(|conn| AgentRepo::new(conn).name_exists(name))
            .map_err(AgentError::Database)?;

        if !exists {
            return Err(AgentError::NotFound(name.to_string()));
        }

        let agent_name = AgentName::new(name);
        ctx.emit(AgentEvents::AgentRemoved(AgentRemoved {
            name: agent_name.clone(),
        }));
        Ok(AgentResponse::AgentRemoved(agent_name))
    }
}

fn normalize_agent_name(name: &str, persona: &str) -> String {
    let suffix = format!(".{persona}");
    if name.ends_with(&suffix) {
        name.to_string()
    } else {
        format!("{name}.{persona}")
    }
}
