use axum::{Json, extract::Path};
use oneiros_model::{PersonaName, PersonaResponses};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<PersonaName>,
) -> Result<Json<PersonaResponses>, Error> {
    let persona = ticket.service().get_persona(&given_name)?;

    Ok(Json(persona))
}
