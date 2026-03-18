use crate::contexts::ProjectContext;

use super::errors::PersonaError;
use super::model::Persona;
use super::repo::PersonaRepo;
use super::responses::PersonaResponse;

pub struct PersonaService;

impl PersonaService {
    pub fn set(ctx: &ProjectContext, persona: Persona) -> Result<PersonaResponse, PersonaError> {
        ctx.emit("persona-set", &persona);
        Ok(PersonaResponse::Set(persona))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<PersonaResponse, PersonaError> {
        let persona = ctx
            .with_db(|conn| PersonaRepo::new(conn).get(name))
            .map_err(PersonaError::Database)?
            .ok_or_else(|| PersonaError::NotFound(name.to_string()))?;
        Ok(PersonaResponse::Found(persona))
    }

    pub fn list(ctx: &ProjectContext) -> Result<PersonaResponse, PersonaError> {
        let personas = ctx
            .with_db(|conn| PersonaRepo::new(conn).list())
            .map_err(PersonaError::Database)?;
        Ok(PersonaResponse::Listed(personas))
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<PersonaResponse, PersonaError> {
        ctx.emit("persona-removed", &serde_json::json!({ "name": name }));
        Ok(PersonaResponse::Removed)
    }
}
