use crate::*;

pub struct ExperienceService;

impl ExperienceService {
    pub async fn create(
        context: &ProjectContext,
        CreateExperience {
            agent,
            sensation,
            description,
        }: &CreateExperience,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let agent_record = AgentRepo::new(context)
            .get(agent)
            .await?
            .ok_or_else(|| ExperienceError::AgentNotFound(agent.clone()))?;

        let experience = Experience::builder()
            .agent_id(agent_record.id)
            .sensation(sensation.clone())
            .description(description.clone())
            .build();

        context
            .emit(ExperienceEvents::ExperienceCreated(experience.clone()))
            .await?;
        Ok(ExperienceResponse::ExperienceCreated(experience))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetExperience,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let experience = ExperienceRepo::new(context)
            .get(&selector.id)
            .await?
            .ok_or_else(|| ExperienceError::NotFound(selector.id))?;
        Ok(ExperienceResponse::ExperienceDetails(experience))
    }

    pub async fn list(
        context: &ProjectContext,
        ListExperiences { agent, filters }: &ListExperiences,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let agent_id = match agent {
            Some(name) => {
                let record = AgentRepo::new(context)
                    .get(name)
                    .await?
                    .ok_or_else(|| ExperienceError::AgentNotFound(name.clone()))?;
                Some(record.id.to_string())
            }
            None => None,
        };

        let listed = ExperienceRepo::new(context)
            .list(agent_id.as_deref(), filters)
            .await?;
        Ok(if listed.total == 0 {
            ExperienceResponse::NoExperiences
        } else {
            ExperienceResponse::Experiences(listed)
        })
    }

    pub async fn update_description(
        context: &ProjectContext,
        UpdateExperienceDescription { id, description }: &UpdateExperienceDescription,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let mut experience = ExperienceRepo::new(context)
            .get(id)
            .await?
            .ok_or_else(|| ExperienceError::NotFound(*id))?;

        experience.description = description.clone();

        context
            .emit(ExperienceEvents::ExperienceDescriptionUpdated(
                ExperienceDescriptionUpdate {
                    id: *id,
                    description: description.clone(),
                },
            ))
            .await?;
        Ok(ExperienceResponse::ExperienceUpdated(experience))
    }

    pub async fn update_sensation(
        context: &ProjectContext,
        UpdateExperienceSensation { id, sensation }: &UpdateExperienceSensation,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let mut experience = ExperienceRepo::new(context)
            .get(id)
            .await?
            .ok_or_else(|| ExperienceError::NotFound(*id))?;

        experience.sensation = sensation.clone();

        context
            .emit(ExperienceEvents::ExperienceSensationUpdated(
                ExperienceSensationUpdate {
                    id: *id,
                    sensation: sensation.clone(),
                },
            ))
            .await?;
        Ok(ExperienceResponse::ExperienceUpdated(experience))
    }
}
