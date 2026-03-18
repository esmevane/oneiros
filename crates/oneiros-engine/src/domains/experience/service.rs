use chrono::Utc;

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
            agent_id: agent,
            sensation,
            description,
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit(ExperienceEvents::ExperienceCreated(experience.clone()));
        Ok(ExperienceResponse::Created(experience))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<ExperienceResponse, ExperienceError> {
        let experience = ctx
            .with_db(|conn| ExperienceRepo::new(conn).get(id))
            .map_err(ExperienceError::Database)?
            .ok_or_else(|| ExperienceError::NotFound(id.to_string()))?;
        Ok(ExperienceResponse::Found(experience))
    }

    pub fn list(
        ctx: &ProjectContext,
        agent: Option<&str>,
    ) -> Result<ExperienceResponse, ExperienceError> {
        let experiences = ctx
            .with_db(|conn| ExperienceRepo::new(conn).list(agent))
            .map_err(ExperienceError::Database)?;
        Ok(ExperienceResponse::Listed(experiences))
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

        experience.description = description.clone();

        ctx.emit(ExperienceEvents::ExperienceDescriptionUpdated(
            ExperienceDescriptionUpdate {
                id: id
                    .parse()
                    .map_err(|e: IdParseError| ExperienceError::Database(e.into()))?,
                description,
            },
        ));
        Ok(ExperienceResponse::Updated(experience))
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

        experience.sensation = sensation.clone();

        ctx.emit(ExperienceEvents::ExperienceSensationUpdated(
            ExperienceSensationUpdate {
                id: id
                    .parse()
                    .map_err(|e: IdParseError| ExperienceError::Database(e.into()))?,
                sensation,
            },
        ));
        Ok(ExperienceResponse::Updated(experience))
    }
}
