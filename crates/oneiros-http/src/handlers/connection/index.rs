use axum::{Json, extract::Query};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(request): Query<ListConnectionsRequest>,
) -> Result<Json<ConnectionResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_connection(ConnectionRequests::ListConnections(request))?;

    Ok(Json(response))
}
