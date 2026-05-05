use crate::*;

pub struct PersonaService;

impl PersonaService {
    pub async fn set(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &SetPersona,
    ) -> Result<PersonaResponse, PersonaError> {
        let SetPersona::V1(set) = request;
        let persona = Persona::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();
        let name = persona.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Persona(PersonaEvents::PersonaSet(
                PersonaSet::builder_v1().persona(persona).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let projected = PersonaRepo::new(scope)
            .fetch(&name)
            .await?
            .ok_or(PersonaError::NotFound(name))?;

        Ok(PersonaResponse::PersonaSet(
            PersonaSetResponse::builder_v1()
                .persona(projected)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetPersona,
    ) -> Result<PersonaResponse, PersonaError> {
        let GetPersona::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let persona = PersonaRepo::new(scope)
            .fetch(&name)
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
        scope: &Scope<AtBookmark>,
        request: &ListPersonas,
    ) -> Result<PersonaResponse, PersonaError> {
        let ListPersonas::V1(listing) = request;
        let listed = PersonaRepo::new(scope).list(&listing.filters).await?;
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
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RemovePersona,
    ) -> Result<PersonaResponse, PersonaError> {
        let RemovePersona::V1(removal) = request;
        let name = removal.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Persona(PersonaEvents::PersonaRemoved(
                PersonaRemoved::builder_v1()
                    .name(name.clone())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        scope
            .config()
            .fetch
            .until_absent(|| async { PersonaRepo::new(scope).get(&name).await })
            .await?;

        Ok(PersonaResponse::PersonaRemoved(
            PersonaRemovedResponse::builder_v1()
                .name(name)
                .build()
                .into(),
        ))
    }
}
