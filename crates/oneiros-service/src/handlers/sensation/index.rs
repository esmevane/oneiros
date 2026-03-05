use axum::Json;
use oneiros_model::SensationResponses;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<SensationResponses>, Error> {
    let sensations = ticket.service().list_sensations()?;

    Ok(Json(sensations))
}
