use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddCognitionRequest>,
) -> Result<(StatusCode, Json<CognitionResponses>), Error> {
    let response = ticket
        .service()
        .dispatch_cognition(CognitionRequests::AddCognition(request))?;

    Ok((StatusCode::CREATED, Json(response)))
}
