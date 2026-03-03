use axum::Json;
use oneiros_model::Level;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Level>>, Error> {
    let levels = ticket.service().list_levels()?;

    Ok(Json(levels))
}
