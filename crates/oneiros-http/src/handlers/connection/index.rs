use axum::{Json, extract::Query};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(request): Query<ListConnectionsRequest>,
) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(ConnectionRequests::ListConnections(request))?,
    ))
}
