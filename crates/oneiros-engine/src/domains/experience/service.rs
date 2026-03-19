use crate::*;

pub struct ExperienceService;

impl ExperienceService {
    pub fn create(
        ctx: &ProjectContext,
        agent: String,
        sensation: String,
        description: String,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let experience = Experience {
            id: ExperienceId::new(),
            agent_id: AgentName::new(agent),
            sensation: SensationName::new(sensation),
            description: Description(description),
            created_at: Timestamp::now(),
        };

        ctx.emit(ExperienceEvents::ExperienceCreated(experience.clone()));
        Ok(ExperienceResponse::ExperienceCreated(experience))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<ExperienceResponse, ExperienceError> {
        let experience = ctx
            .with_db(|conn| ExperienceRepo::new(conn).get(id))
            .map_err(ExperienceError::Database)?
            .ok_or_else(|| ExperienceError::NotFound(id.to_string()))?;
        Ok(ExperienceResponse::ExperienceDetails(experience))
    }

    pub fn list(
        ctx: &ProjectContext,
        agent: Option<&str>,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let experiences = ctx
            .with_db(|conn| ExperienceRepo::new(conn).list(agent))
            .map_err(ExperienceError::Database)?;
        Ok(if experiences.is_empty() {
            ExperienceResponse::NoExperiences
        } else {
            ExperienceResponse::Experiences(experiences)
        })
    }

    pub fn update_description(
        ctx: &ProjectContext,
        id: &str,
        description: String,
    ) -> Result<ExperienceResponse, ExperienceError> {
        // Confirm existence and return the updated record.
        let mut experience = ctx
            .with_db(|conn| ExperienceRepo::new(conn).get(id))
            .map_err(ExperienceError::Database)?
            .ok_or_else(|| ExperienceError::NotFound(id.to_string()))?;

        experience.description = Description(description.clone());

        ctx.emit(ExperienceEvents::ExperienceDescriptionUpdated(
            ExperienceDescriptionUpdate {
                id: id
                    .parse()
                    .map_err(|e: IdParseError| ExperienceError::Database(e.into()))?,
                description: Description(description),
            },
        ));
        Ok(ExperienceResponse::ExperienceUpdated(experience))
    }

    pub fn update_sensation(
        ctx: &ProjectContext,
        id: &str,
        sensation: String,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let mut experience = ctx
            .with_db(|conn| ExperienceRepo::new(conn).get(id))
            .map_err(ExperienceError::Database)?
            .ok_or_else(|| ExperienceError::NotFound(id.to_string()))?;

        experience.sensation = SensationName::new(&sensation);

        ctx.emit(ExperienceEvents::ExperienceSensationUpdated(
            ExperienceSensationUpdate {
                id: id
                    .parse()
                    .map_err(|e: IdParseError| ExperienceError::Database(e.into()))?,
                sensation: SensationName::new(sensation),
            },
        ));
        Ok(ExperienceResponse::ExperienceUpdated(experience))
    }
}
