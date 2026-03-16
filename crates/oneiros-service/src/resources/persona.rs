use oneiros_model::*;

use crate::*;

pub struct PersonaStore;

impl Dispatch<PersonaRequests> for PersonaStore {
    type Response = PersonaResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, PersonaRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            PersonaRequests::SetPersona(persona) => {
                let event = Events::Persona(PersonaEvents::PersonaSet(persona.clone()));
                context.scope.effects().emit(&event)?;
                Ok(PersonaResponses::PersonaSet(persona))
            }
            PersonaRequests::ListPersonas(_) => {
                Ok(PersonaResponses::PersonasListed(db.list_personas()?))
            }
            PersonaRequests::GetPersona(request) => {
                let persona = db
                    .get_persona(&request.name)?
                    .ok_or(NotFound::Persona(request.name))?;
                Ok(PersonaResponses::PersonaFound(persona))
            }
            PersonaRequests::RemovePersona(request) => {
                let event = Events::Persona(PersonaEvents::PersonaRemoved(SelectPersonaByName {
                    name: request.name,
                }));
                context.scope.effects().emit(&event)?;
                Ok(PersonaResponses::PersonaRemoved)
            }
        }
    }
}
