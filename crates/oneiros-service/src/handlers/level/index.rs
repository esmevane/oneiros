use axum::Json;
use oneiros_model::Level;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Level>>, Error> {
    let levels = ticket.db.list_levels()?;

    Ok(Json(levels))
}
