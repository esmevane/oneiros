use crate::*;

pub struct ExperienceService;

impl ExperienceService {
    pub async fn create(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &CreateExperience,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let CreateExperience::V1(creation) = request;
        let agent_record = AgentRepo::new(scope)
            .fetch(&creation.agent)
            .await?
            .ok_or_else(|| ExperienceError::AgentNotFound(creation.agent.clone()))?;

        let experience = Experience::builder()
            .agent_id(agent_record.id)
            .sensation(creation.sensation.clone())
            .description(creation.description.clone())
            .build();
        let id = experience.id;

        let new_event = NewEvent::builder()
            .data(Events::Experience(ExperienceEvents::ExperienceCreated(
                ExperienceCreated::builder_v1()
                    .experience(experience)
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let stored = ExperienceRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(ExperienceError::NotFound(id))?;

        Ok(ExperienceResponse::ExperienceCreated(
            ExperienceCreatedResponse::builder_v1()
                .experience(stored)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetExperience,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let GetExperience::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let experience = ExperienceRepo::new(scope)
            .fetch(&id)
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
        scope: &Scope<AtBookmark>,
        request: &ListExperiences,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let ListExperiences::V1(listing) = request;
        let agent_id = match &listing.agent {
            Some(name) => {
                let record = AgentRepo::new(scope)
                    .fetch(name)
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

        let results = SearchRepo::new(scope)
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
        let items = ExperienceRepo::new(scope).get_many(&ids).await?;

        Ok(ExperienceResponse::Experiences(
            ExperiencesResponse::builder_v1()
                .items(items)
                .total(results.total)
                .build()
                .into(),
        ))
    }

    pub async fn update_description(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &UpdateExperienceDescription,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let UpdateExperienceDescription::V1(update) = request;
        // Validate the experience exists before dispatching the update.
        ExperienceRepo::new(scope)
            .fetch(&update.id)
            .await?
            .ok_or(ExperienceError::NotFound(update.id))?;

        let new_event = NewEvent::builder()
            .data(Events::Experience(
                ExperienceEvents::ExperienceDescriptionUpdated(
                    ExperienceDescriptionUpdated::builder_v1()
                        .id(update.id)
                        .description(update.description.clone())
                        .build()
                        .into(),
                ),
            ))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        // Read back the updated record. Fetch polls until the projection
        // reflects the new description.
        let updated = scope
            .config()
            .fetch
            .eventual(|| async {
                ExperienceRepo::new(scope)
                    .get(&update.id)
                    .await
                    .map(|opt| opt.filter(|exp| exp.description == update.description))
            })
            .await?
            .ok_or(ExperienceError::NotFound(update.id))?;

        Ok(ExperienceResponse::ExperienceUpdated(
            ExperienceUpdatedResponse::builder_v1()
                .experience(updated)
                .build()
                .into(),
        ))
    }

    pub async fn update_sensation(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &UpdateExperienceSensation,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let UpdateExperienceSensation::V1(update) = request;
        ExperienceRepo::new(scope)
            .fetch(&update.id)
            .await?
            .ok_or(ExperienceError::NotFound(update.id))?;

        let new_event = NewEvent::builder()
            .data(Events::Experience(
                ExperienceEvents::ExperienceSensationUpdated(
                    ExperienceSensationUpdated::builder_v1()
                        .id(update.id)
                        .sensation(update.sensation.clone())
                        .build()
                        .into(),
                ),
            ))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let updated = scope
            .config()
            .fetch
            .eventual(|| async {
                ExperienceRepo::new(scope)
                    .get(&update.id)
                    .await
                    .map(|opt| opt.filter(|exp| exp.sensation == update.sensation))
            })
            .await?
            .ok_or(ExperienceError::NotFound(update.id))?;

        Ok(ExperienceResponse::ExperienceUpdated(
            ExperienceUpdatedResponse::builder_v1()
                .experience(updated)
                .build()
                .into(),
        ))
    }
}
