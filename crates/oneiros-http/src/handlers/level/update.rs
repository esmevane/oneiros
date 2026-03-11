use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(level): Json<Level>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(LevelRequests::SetLevel(level))?))
}
