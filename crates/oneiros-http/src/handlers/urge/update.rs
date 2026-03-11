use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(urge): Json<Urge>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(UrgeRequests::SetUrge(urge))?))
}
