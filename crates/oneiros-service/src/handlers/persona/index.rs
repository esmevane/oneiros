use axum::Json;
use oneiros_model::PersonaRecord;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<PersonaRecord>>, Error> {
    let personas = ticket.db.list_personas()?;

    Ok(Json(personas))
}
