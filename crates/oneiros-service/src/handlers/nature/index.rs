use axum::Json;
use oneiros_model::NatureResponses;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<NatureResponses>, Error> {
    let natures = ticket.service().list_natures()?;

    Ok(Json(natures))
}
