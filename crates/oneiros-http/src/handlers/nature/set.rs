use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(nature): Json<Nature>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(NatureRequests::SetNature(nature))?))
}
