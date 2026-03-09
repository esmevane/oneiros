use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateConnectionRequest>,
) -> Result<(StatusCode, Json<ConnectionResponses>), Error> {
    let response = ticket
        .service()
        .dispatch_connection(ConnectionRequests::CreateConnection(request))?;

    Ok((StatusCode::CREATED, Json(response)))
}
