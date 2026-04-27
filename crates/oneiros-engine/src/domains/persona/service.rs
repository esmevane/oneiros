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
        let persona = Persona::Current(
            Persona::build_v1()
                .name(name.clone())
                .description(description.clone())
                .prompt(prompt.clone())
                .build(),
        );
        context.emit(PersonaEvents::PersonaSet(persona)).await?;
        Ok(PersonaResponse::PersonaSet(name.clone()))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetPersona,
    ) -> Result<PersonaResponse, PersonaError> {
        let name = selector.key.resolve()?;
        let persona = PersonaRepo::new(context)
            .get(&name)
            .await?
            .ok_or(PersonaError::NotFound(name))?;
        let ref_token = RefToken::new(Ref::persona(persona.name().clone()));
        Ok(PersonaResponse::PersonaDetails(
            Response::new(persona).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        ListPersonas { filters }: &ListPersonas,
    ) -> Result<PersonaResponse, PersonaError> {
        let listed = PersonaRepo::new(context).list(filters).await?;
        if listed.total == 0 {
            Ok(PersonaResponse::NoPersonas)
        } else {
            Ok(PersonaResponse::Personas(listed.map(|e| {
                let ref_token = RefToken::new(Ref::persona(e.name().clone()));
                Response::new(e).with_ref_token(ref_token)
            })))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemovePersona,
    ) -> Result<PersonaResponse, PersonaError> {
        context
            .emit(PersonaEvents::PersonaRemoved(PersonaRemoved::Current(
                PersonaRemovedV1 {
                    name: selector.name.clone(),
                },
            )))
            .await?;
        Ok(PersonaResponse::PersonaRemoved(selector.name.clone()))
    }
}
