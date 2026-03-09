use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<PersonaName>,
) -> Result<Json<PersonaResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_persona(PersonaRequests::GetPersona(GetPersonaRequest { name }))?;

    Ok(Json(response))
}
