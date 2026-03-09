use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<PersonaResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_persona(PersonaRequests::ListPersonas(ListPersonasRequest))?;

    Ok(Json(response))
}
