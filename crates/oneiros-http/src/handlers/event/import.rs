use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(events): Json<Vec<ImportEvent>>,
) -> Result<Json<ImportResponse>, Error> {
    let response = ticket.service().import_events(&events)?;

    Ok(Json(response))
}
