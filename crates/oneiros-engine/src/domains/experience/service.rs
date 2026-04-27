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

        let experience = Experience::Current(
            Experience::build_v1()
                .agent_id(agent_record.id())
                .sensation(sensation.clone())
                .description(description.clone())
                .build(),
        );

        context
            .emit(ExperienceEvents::ExperienceCreated(experience.clone()))
            .await?;
        let ref_token = RefToken::new(Ref::experience(experience.id()));
        Ok(ExperienceResponse::ExperienceCreated(
            Response::new(experience).with_ref_token(ref_token),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetExperience,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let id = selector.key.resolve()?;
        let experience = ExperienceRepo::new(context)
            .get(&id)
            .await?
            .ok_or(ExperienceError::NotFound(id))?;
        let ref_token = RefToken::new(Ref::experience(experience.id()));
        Ok(ExperienceResponse::ExperienceDetails(
            Response::new(experience).with_ref_token(ref_token),
        ))
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
                Some(record.id().to_string())
            }
            None => None,
        };

        let listed = ExperienceRepo::new(context)
            .list(agent_id.as_deref(), filters)
            .await?;
        Ok(if listed.total == 0 {
            ExperienceResponse::NoExperiences
        } else {
            ExperienceResponse::Experiences(listed.map(|e| {
                let ref_token = RefToken::new(Ref::experience(e.id()));
                Response::new(e).with_ref_token(ref_token)
            }))
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

        experience.set_description(description.clone());

        context
            .emit(ExperienceEvents::ExperienceDescriptionUpdated(
                ExperienceDescriptionUpdate::Current(ExperienceDescriptionUpdateV1 {
                    id: *id,
                    description: description.clone(),
                }),
            ))
            .await?;
        let ref_token = RefToken::new(Ref::experience(experience.id()));
        Ok(ExperienceResponse::ExperienceUpdated(
            Response::new(experience).with_ref_token(ref_token),
        ))
    }

    pub async fn update_sensation(
        context: &ProjectContext,
        UpdateExperienceSensation { id, sensation }: &UpdateExperienceSensation,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let mut experience = ExperienceRepo::new(context)
            .get(id)
            .await?
            .ok_or_else(|| ExperienceError::NotFound(*id))?;

        experience.set_sensation(sensation.clone());

        context
            .emit(ExperienceEvents::ExperienceSensationUpdated(
                ExperienceSensationUpdate::Current(ExperienceSensationUpdateV1 {
                    id: *id,
                    sensation: sensation.clone(),
                }),
            ))
            .await?;
        let ref_token = RefToken::new(Ref::experience(experience.id()));
        Ok(ExperienceResponse::ExperienceUpdated(
            Response::new(experience).with_ref_token(ref_token),
        ))
    }
}
