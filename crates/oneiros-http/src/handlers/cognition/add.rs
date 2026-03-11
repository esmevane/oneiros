use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddCognitionRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    Ok((
        StatusCode::CREATED,
        Json(ticket.dispatch(CognitionRequests::AddCognition(request))?),
    ))
}
