use crate::*;

pub struct PersonaService;

impl PersonaService {
    pub async fn set(
        context: &ProjectContext,
        SetPersona {
            name,
            description,
            prompt,
        }: &SetPersona,
    ) -> Result<PersonaResponse, PersonaError> {
        let persona = Persona::builder()
            .name(name.clone())
            .description(description.clone())
            .prompt(prompt.clone())
            .build();
        context.emit(PersonaEvents::PersonaSet(persona)).await?;
        Ok(PersonaResponse::PersonaSet(name.clone()))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetPersona,
    ) -> Result<PersonaResponse, PersonaError> {
        let persona = PersonaRepo::new(context)
            .get(&selector.name)
            .await?
            .ok_or_else(|| PersonaError::NotFound(selector.name.clone()))?;
        Ok(PersonaResponse::PersonaDetails(persona))
    }

    pub async fn list(
        context: &ProjectContext,
        ListPersonas { filters }: &ListPersonas,
    ) -> Result<PersonaResponse, PersonaError> {
        let listed = PersonaRepo::new(context).list(filters).await?;
        if listed.total == 0 {
            Ok(PersonaResponse::NoPersonas)
        } else {
            Ok(PersonaResponse::Personas(listed))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemovePersona,
    ) -> Result<PersonaResponse, PersonaError> {
        context
            .emit(PersonaEvents::PersonaRemoved(PersonaRemoved {
                name: selector.name.clone(),
            }))
            .await?;
        Ok(PersonaResponse::PersonaRemoved(selector.name.clone()))
    }
}
