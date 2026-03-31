use crate::*;

pub struct AgentService;

impl AgentService {
    pub async fn create(
        context: &ProjectContext,
        CreateAgent {
            name,
            persona,
            description,
            prompt,
        }: &CreateAgent,
    ) -> Result<AgentResponse, AgentError> {
        // Cross-resource validation: persona must exist
        //
        if PersonaRepo::new(context).get(persona).await?.is_none() {
            return Err(AgentError::PersonaNotFound(persona.clone()));
        }

        // Normalize name: append .persona if not already present
        let normalized_name = name.normalize_with(persona);

        // Validate name uniqueness
        if AgentRepo::new(context)
            .name_exists(&normalized_name)
            .await?
        {
            return Err(AgentError::Conflict(normalized_name));
        }

        let agent = Agent::builder()
            .name(normalized_name.clone())
            .persona(persona.clone())
            .description(description.clone())
            .prompt(prompt.clone())
            .build();

        context.emit(AgentEvents::AgentCreated(agent)).await?;

        Ok(AgentResponse::AgentCreated(normalized_name))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetAgent,
    ) -> Result<AgentResponse, AgentError> {
        let agent = AgentRepo::new(context)
            .get(&selector.name)
            .await?
            .ok_or_else(|| AgentError::NotFound(selector.name.clone()))?;

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
        UpdateAgent {
            name,
            persona,
            description,
            prompt,
        }: &UpdateAgent,
    ) -> Result<AgentResponse, AgentError> {
        let existing = AgentRepo::new(context)
            .get(name)
            .await?
            .ok_or_else(|| AgentError::NotFound(name.clone()))?;

        let agent = Agent::builder()
            .id(existing.id)
            .name(name.clone())
            .persona(persona.clone())
            .description(description.clone())
            .prompt(prompt.clone())
            .build();

        context.emit(AgentEvents::AgentUpdated(agent)).await?;

        Ok(AgentResponse::AgentUpdated(name.clone()))
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveAgent,
    ) -> Result<AgentResponse, AgentError> {
        let exists = AgentRepo::new(context).name_exists(&selector.name).await?;

        if !exists {
            return Err(AgentError::NotFound(selector.name.clone()));
        }

        context
            .emit(AgentEvents::AgentRemoved(AgentRemoved {
                name: selector.name.clone(),
            }))
            .await?;

        Ok(AgentResponse::AgentRemoved(selector.name.clone()))
    }
}
