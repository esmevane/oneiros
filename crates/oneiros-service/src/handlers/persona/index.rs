use axum::Json;
use oneiros_model::Persona;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Persona>>, Error> {
    let personas = ticket.db.list_personas()?;

    Ok(Json(personas))
}
