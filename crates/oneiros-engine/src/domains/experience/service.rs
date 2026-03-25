use crate::*;

pub struct ExperienceService;

impl ExperienceService {
    pub async fn create(
        context: &ProjectContext,
        agent: &AgentName,
        sensation: SensationName,
        description: Description,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let agent_record = context
            .with_db(|conn| AgentRepo::new(conn).get(agent))?
            .ok_or_else(|| ExperienceError::AgentNotFound(agent.clone()))?;

        let experience = Experience::builder()
            .agent_id(agent_record.id)
            .sensation(sensation)
            .description(description)
            .build();

        context
            .emit(ExperienceEvents::ExperienceCreated(experience.clone()))
            .await?;
        Ok(ExperienceResponse::ExperienceCreated(experience))
    }

    pub fn get(
        context: &ProjectContext,
        id: &ExperienceId,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let experience = context
            .with_db(|conn| ExperienceRepo::new(conn).get(id))?
            .ok_or_else(|| ExperienceError::NotFound(*id))?;
        Ok(ExperienceResponse::ExperienceDetails(experience))
    }

    pub fn list(
        context: &ProjectContext,
        agent: Option<&AgentName>,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let agent_id = agent
            .map(|name| {
                context
                    .with_db(|conn| AgentRepo::new(conn).get(name))?
                    .map(|a| a.id.to_string())
                    .ok_or_else(|| ExperienceError::AgentNotFound(name.clone()))
            })
            .transpose()?;

        let experiences = context
            .with_db(|conn| ExperienceRepo::new(conn).list(agent_id.as_deref()))
            .map_err(ExperienceError::Database)?;
        Ok(if experiences.is_empty() {
            ExperienceResponse::NoExperiences
        } else {
            ExperienceResponse::Experiences(experiences)
        })
    }

    pub async fn update_description(
        context: &ProjectContext,
        id: &ExperienceId,
        description: Description,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let mut experience = context
            .with_db(|conn| ExperienceRepo::new(conn).get(id))?
            .ok_or_else(|| ExperienceError::NotFound(*id))?;

        experience.description = description.clone();

        context
            .emit(ExperienceEvents::ExperienceDescriptionUpdated(
                ExperienceDescriptionUpdate {
                    id: *id,
                    description,
                },
            ))
            .await?;
        Ok(ExperienceResponse::ExperienceUpdated(experience))
    }

    pub async fn update_sensation(
        context: &ProjectContext,
        id: &ExperienceId,
        sensation: SensationName,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let mut experience = context
            .with_db(|conn| ExperienceRepo::new(conn).get(id))?
            .ok_or_else(|| ExperienceError::NotFound(*id))?;

        experience.sensation = sensation.clone();

        context
            .emit(ExperienceEvents::ExperienceSensationUpdated(
                ExperienceSensationUpdate { id: *id, sensation },
            ))
            .await?;
        Ok(ExperienceResponse::ExperienceUpdated(experience))
    }
}
