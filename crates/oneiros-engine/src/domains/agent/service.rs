use crate::*;

pub struct AgentService;

impl AgentService {
    pub fn create(
        context: &ProjectContext,
        agent_name: AgentName,
        persona: PersonaName,
        description: Description,
        prompt: Prompt,
    ) -> Result<AgentResponse, AgentError> {
        // Cross-resource validation: persona must exist
        let persona_exists = context.with_db(|conn| PersonaRepo::new(conn).get(&persona))?;

        if persona_exists.is_none() {
            return Err(AgentError::PersonaNotFound(persona));
        }

        // Normalize name: append .persona if not already present
        let normalized_name = agent_name.normalize_with(&persona);

        // Validate name uniqueness
        let already_exists =
            context.with_db(|conn| AgentRepo::new(conn).name_exists(&normalized_name))?;

        if already_exists {
            return Err(AgentError::Conflict(normalized_name));
        }

        let agent = Agent::builder()
            .name(normalized_name.clone())
            .persona(persona)
            .description(description)
            .prompt(prompt)
            .build();

        context.emit(AgentEvents::AgentCreated(agent));

        Ok(AgentResponse::AgentCreated(normalized_name))
    }

    pub fn get(context: &ProjectContext, name: &AgentName) -> Result<AgentResponse, AgentError> {
        let agent = context
            .with_db(|conn| AgentRepo::new(conn).get(name))?
            .ok_or_else(|| AgentError::NotFound(name.clone()))?;

        Ok(AgentResponse::AgentDetails(agent))
    }

    pub fn list(context: &ProjectContext) -> Result<AgentResponse, AgentError> {
        let agents = context.with_db(|conn| AgentRepo::new(conn).list())?;

        if agents.is_empty() {
            Ok(AgentResponse::NoAgents)
        } else {
            Ok(AgentResponse::Agents(agents))
        }
    }

    pub fn update(
        ctx: &ProjectContext,
        agent_name: AgentName,
        persona: PersonaName,
        description: Description,
        prompt: Prompt,
    ) -> Result<AgentResponse, AgentError> {
        let existing = ctx
            .with_db(|conn| AgentRepo::new(conn).get(&agent_name))?
            .ok_or_else(|| AgentError::NotFound(agent_name.clone()))?;

        let agent = Agent::builder()
            .id(existing.id)
            .name(agent_name.clone())
            .persona(persona)
            .description(description)
            .prompt(prompt)
            .build();

        ctx.emit(AgentEvents::AgentUpdated(agent));
        Ok(AgentResponse::AgentUpdated(agent_name))
    }

    pub fn remove(
        context: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<AgentResponse, AgentError> {
        let exists = context.with_db(|conn| AgentRepo::new(conn).name_exists(agent_name))?;

        if !exists {
            return Err(AgentError::NotFound(agent_name.clone()));
        }

        context.emit(AgentEvents::AgentRemoved(AgentRemoved {
            name: agent_name.clone(),
        }));

        Ok(AgentResponse::AgentRemoved(agent_name.clone()))
    }
}
