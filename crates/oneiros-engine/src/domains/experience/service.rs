use crate::*;

pub struct ExperienceService;

impl ExperienceService {
    pub async fn create(
        context: &ProjectLog,
        request: &CreateExperience,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let CreateExperience::V1(creation) = request;
        let agent_record = AgentRepo::new(context.scope()?)
            .fetch(&creation.agent, &context.config.fetch)
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
        context: &ProjectLog,
        request: &GetExperience,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let GetExperience::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let experience = ExperienceRepo::new(context.scope()?)
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
        context: &ProjectLog,
        request: &ListExperiences,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let ListExperiences::V1(listing) = request;
        let agent_id = match &listing.agent {
            Some(name) => {
                let record = AgentRepo::new(context.scope()?)
                    .fetch(name, &context.config.fetch)
                    .await?
                    .ok_or_else(|| ExperienceError::AgentNotFound(name.clone()))?;
                Some(record.id)
            }
            None => None,
        };

        let search_query = SearchQuery::builder_v1()
            .kind(SearchKind::Experience)
            .maybe_sensation(listing.sensation.clone())
            .maybe_query(listing.query.clone())
            .filters(listing.filters)
            .build();

        let results = SearchRepo::new(context.scope()?)
            .search(&search_query, agent_id.as_ref())
            .await?;

        if results.total == 0 {
            return Ok(ExperienceResponse::NoExperiences);
        }

        let ids: Vec<ExperienceId> = results
            .hits
            .iter()
            .filter_map(|hit| match &hit.resource_ref {
                Ref::V0(Resource::Experience(id)) => Some(*id),
                _ => None,
            })
            .collect();
        let items = ExperienceRepo::new(context.scope()?).get_many(&ids).await?;

        Ok(ExperienceResponse::Experiences(
            ExperiencesResponse::builder_v1()
                .items(items)
                .total(results.total)
                .build()
                .into(),
        ))
    }

    pub async fn update_description(
        context: &ProjectLog,
        request: &UpdateExperienceDescription,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let UpdateExperienceDescription::V1(update) = request;
        let mut experience = ExperienceRepo::new(context.scope()?)
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
        context: &ProjectLog,
        request: &UpdateExperienceSensation,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let UpdateExperienceSensation::V1(update) = request;
        let mut experience = ExperienceRepo::new(context.scope()?)
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
