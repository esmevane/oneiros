use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<ReplayResponse>, Error> {
    let response = ticket.service().replay()?;

    Ok(Json(response))
}
