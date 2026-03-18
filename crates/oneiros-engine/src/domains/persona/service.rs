use crate::*;

pub struct PersonaService;

impl PersonaService {
    pub fn set(ctx: &ProjectContext, persona: Persona) -> Result<PersonaResponse, PersonaError> {
        let name = persona.name.clone();
        ctx.emit(PersonaEvents::PersonaSet(persona));
        Ok(PersonaResponse::PersonaSet(name))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<PersonaResponse, PersonaError> {
        let persona = ctx
            .with_db(|conn| PersonaRepo::new(conn).get(name))
            .map_err(PersonaError::Database)?
            .ok_or_else(|| PersonaError::NotFound(name.to_string()))?;
        Ok(PersonaResponse::PersonaDetails(persona))
    }

    pub fn list(ctx: &ProjectContext) -> Result<PersonaResponse, PersonaError> {
        let personas = ctx
            .with_db(|conn| PersonaRepo::new(conn).list())
            .map_err(PersonaError::Database)?;
        if personas.is_empty() {
            Ok(PersonaResponse::NoPersonas)
        } else {
            Ok(PersonaResponse::Personas(personas))
        }
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<PersonaResponse, PersonaError> {
        let persona_name = PersonaName::new(name);
        ctx.emit(PersonaEvents::PersonaRemoved(PersonaRemoved {
            name: persona_name.clone(),
        }));
        Ok(PersonaResponse::PersonaRemoved(persona_name))
    }
}
