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
        if PersonaRepo::new(&context.db()?).get(&persona)?.is_none() {
            return Err(AgentError::PersonaNotFound(persona));
        }

        // Normalize name: append .persona if not already present
        let normalized_name = agent_name.normalize_with(&persona);

        // Validate name uniqueness
        if AgentRepo::new(&context.db()?).name_exists(&normalized_name)? {
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

    pub fn get(context: &ProjectContext, name: &AgentName) -> Result<AgentResponse, AgentError> {
        let db = context.db()?;
        let agent_repo = AgentRepo::new(&db);
        let agent = agent_repo
            .get(name)?
            .ok_or_else(|| AgentError::NotFound(name.clone()))?;

        Ok(AgentResponse::AgentDetails(agent))
    }

    pub fn list(context: &ProjectContext) -> Result<AgentResponse, AgentError> {
        let db = context.db()?;
        let agent_repo = AgentRepo::new(&db);
        let agents = agent_repo.list()?;

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
        let existing = AgentRepo::new(&context.db()?)
            .get(&agent_name)?
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
        let db = context.db()?;
        let exists = AgentRepo::new(&db).name_exists(agent_name)?;

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
