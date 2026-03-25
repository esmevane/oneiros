use crate::*;

pub struct PersonaService;

impl PersonaService {
    pub async fn set(
        context: &ProjectContext,
        persona: Persona,
    ) -> Result<PersonaResponse, PersonaError> {
        let name = persona.name.clone();
        context.emit(PersonaEvents::PersonaSet(persona)).await?;
        Ok(PersonaResponse::PersonaSet(name))
    }

    pub async fn get(
        context: &ProjectContext,
        name: &PersonaName,
    ) -> Result<PersonaResponse, PersonaError> {
        let persona = PersonaRepo::new(context)
            .get(name)
            .await?
            .ok_or_else(|| PersonaError::NotFound(name.clone()))?;
        Ok(PersonaResponse::PersonaDetails(persona))
    }

    pub async fn list(context: &ProjectContext) -> Result<PersonaResponse, PersonaError> {
        let personas = PersonaRepo::new(context).list().await?;
        if personas.is_empty() {
            Ok(PersonaResponse::NoPersonas)
        } else {
            Ok(PersonaResponse::Personas(personas))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        persona_name: &PersonaName,
    ) -> Result<PersonaResponse, PersonaError> {
        context
            .emit(PersonaEvents::PersonaRemoved(PersonaRemoved {
                name: persona_name.clone(),
            }))
            .await?;
        Ok(PersonaResponse::PersonaRemoved(persona_name.clone()))
    }
}
