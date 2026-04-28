use crate::*;

pub struct ExperienceService;

impl ExperienceService {
    pub async fn create(
        context: &ProjectContext,
        request: &CreateExperience,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let CreateExperience::V1(creation) = request;
        let agent_record = AgentRepo::new(context)
            .get(&creation.agent)
            .await?
            .ok_or_else(|| ExperienceError::AgentNotFound(creation.agent.clone()))?;

        let experience = Experience::builder()
            .agent_id(agent_record.id)
            .sensation(creation.sensation.clone())
            .description(creation.description.clone())
            .build();

        context
            .emit(ExperienceEvents::ExperienceCreated(
                ExperienceCreated::builder_v1()
                    .experience(experience.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(ExperienceResponse::ExperienceCreated(
            ExperienceCreatedResponse::builder_v1()
                .experience(experience)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        request: &GetExperience,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let GetExperience::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let experience = ExperienceRepo::new(context)
            .get(&id)
            .await?
            .ok_or(ExperienceError::NotFound(id))?;
        Ok(ExperienceResponse::ExperienceDetails(
            ExperienceDetailsResponse::builder_v1()
                .experience(experience)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        request: &ListExperiences,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let ListExperiences::V1(listing) = request;
        let agent_id = match &listing.agent {
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
            .list(agent_id.as_deref(), &listing.filters)
            .await?;
        Ok(if listed.total == 0 {
            ExperienceResponse::NoExperiences
        } else {
            ExperienceResponse::Experiences(
                ExperiencesResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            )
        })
    }

    pub async fn update_description(
        context: &ProjectContext,
        request: &UpdateExperienceDescription,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let UpdateExperienceDescription::V1(update) = request;
        let mut experience = ExperienceRepo::new(context)
            .get(&update.id)
            .await?
            .ok_or_else(|| ExperienceError::NotFound(update.id))?;

        experience.description = update.description.clone();

        context
            .emit(ExperienceEvents::ExperienceDescriptionUpdated(
                ExperienceDescriptionUpdated::builder_v1()
                    .id(update.id)
                    .description(update.description.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(ExperienceResponse::ExperienceUpdated(
            ExperienceUpdatedResponse::builder_v1()
                .experience(experience)
                .build()
                .into(),
        ))
    }

    pub async fn update_sensation(
        context: &ProjectContext,
        request: &UpdateExperienceSensation,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let UpdateExperienceSensation::V1(update) = request;
        let mut experience = ExperienceRepo::new(context)
            .get(&update.id)
            .await?
            .ok_or_else(|| ExperienceError::NotFound(update.id))?;

        experience.sensation = update.sensation.clone();

        context
            .emit(ExperienceEvents::ExperienceSensationUpdated(
                ExperienceSensationUpdated::builder_v1()
                    .id(update.id)
                    .sensation(update.sensation.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(ExperienceResponse::ExperienceUpdated(
            ExperienceUpdatedResponse::builder_v1()
                .experience(experience)
                .build()
                .into(),
        ))
    }
}
