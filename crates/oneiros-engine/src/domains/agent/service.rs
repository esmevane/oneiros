use crate::*;

pub struct AgentService;

impl AgentService {
    pub async fn create(
        context: &ProjectContext,
        agent_name: AgentName,
        persona: PersonaName,
        description: Description,
        prompt: Prompt,
    ) -> Result<AgentResponse, AgentError> {
        // Cross-resource validation: persona must exist
        //
        if PersonaRepo::new(context).get(&persona).await?.is_none() {
            return Err(AgentError::PersonaNotFound(persona));
        }

        // Normalize name: append .persona if not already present
        let normalized_name = agent_name.normalize_with(&persona);

        // Validate name uniqueness
        if AgentRepo::new(context)
            .name_exists(&normalized_name)
            .await?
        {
            return Err(AgentError::Conflict(normalized_name));
        }

        let agent = Agent::builder()
            .name(normalized_name.clone())
            .persona(persona)
            .description(description)
            .prompt(prompt)
            .build();

        context.emit(AgentEvents::AgentCreated(agent)).await?;

        Ok(AgentResponse::AgentCreated(normalized_name))
    }

    pub async fn get(
        context: &ProjectContext,
        name: &AgentName,
    ) -> Result<AgentResponse, AgentError> {
        let agent = AgentRepo::new(context)
            .get(name)
            .await?
            .ok_or_else(|| AgentError::NotFound(name.clone()))?;

        Ok(AgentResponse::AgentDetails(agent))
    }

    pub async fn list(context: &ProjectContext) -> Result<AgentResponse, AgentError> {
        let agents = AgentRepo::new(context).list().await?;

        if agents.is_empty() {
            Ok(AgentResponse::NoAgents)
        } else {
            Ok(AgentResponse::Agents(agents))
        }
    }

    pub async fn update(
        context: &ProjectContext,
        agent_name: AgentName,
        persona: PersonaName,
        description: Description,
        prompt: Prompt,
    ) -> Result<AgentResponse, AgentError> {
        let existing = AgentRepo::new(context)
            .get(&agent_name)
            .await?
            .ok_or_else(|| AgentError::NotFound(agent_name.clone()))?;

        let agent = Agent::builder()
            .id(existing.id)
            .name(agent_name.clone())
            .persona(persona)
            .description(description)
            .prompt(prompt)
            .build();

        context.emit(AgentEvents::AgentUpdated(agent)).await?;

        Ok(AgentResponse::AgentUpdated(agent_name))
    }

    pub async fn remove(
        context: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<AgentResponse, AgentError> {
        let exists = AgentRepo::new(context).name_exists(agent_name).await?;

        if !exists {
            return Err(AgentError::NotFound(agent_name.clone()));
        }

        context
            .emit(AgentEvents::AgentRemoved(AgentRemoved {
                name: agent_name.clone(),
            }))
            .await?;

        Ok(AgentResponse::AgentRemoved(agent_name.clone()))
    }
}
