use crate::*;

pub struct PersonaService;

impl PersonaService {
    pub async fn set(
        context: &ProjectLog,
        request: &SetPersona,
    ) -> Result<PersonaResponse, PersonaError> {
        let SetPersona::V1(set) = request;
        let persona = Persona::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();

        context
            .emit(PersonaEvents::PersonaSet(
                PersonaSet::builder_v1()
                    .persona(persona.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(PersonaResponse::PersonaSet(
            PersonaSetResponse::builder_v1()
                .persona(persona)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        context: &ProjectLog,
        request: &GetPersona,
    ) -> Result<PersonaResponse, PersonaError> {
        let GetPersona::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let persona = PersonaRepo::new(context.scope()?)
            .fetch(&name, &context.config.fetch)
            .await?
            .ok_or(PersonaError::NotFound(name))?;
        Ok(PersonaResponse::PersonaDetails(
            PersonaDetailsResponse::builder_v1()
                .persona(persona)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        context: &ProjectLog,
        request: &ListPersonas,
    ) -> Result<PersonaResponse, PersonaError> {
        let ListPersonas::V1(listing) = request;
        let listed = PersonaRepo::new(context.scope()?)
            .list(&listing.filters)
            .await?;
        if listed.total == 0 {
            Ok(PersonaResponse::NoPersonas)
        } else {
            Ok(PersonaResponse::Personas(
                PersonasResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            ))
        }
    }

    pub async fn remove(
        context: &ProjectLog,
        request: &RemovePersona,
    ) -> Result<PersonaResponse, PersonaError> {
        let RemovePersona::V1(removal) = request;
        context
            .emit(PersonaEvents::PersonaRemoved(
                PersonaRemoved::builder_v1()
                    .name(removal.name.clone())
                    .build()
                    .into(),
            ))
            .await?;
        Ok(PersonaResponse::PersonaRemoved(
            PersonaRemovedResponse::builder_v1()
                .name(removal.name.clone())
                .build()
                .into(),
        ))
    }
}
