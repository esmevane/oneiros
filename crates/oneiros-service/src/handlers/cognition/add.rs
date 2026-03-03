use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddCognitionRequest>,
) -> Result<(StatusCode, Json<Cognition>), Error> {
    let cognition = ticket.service().add_cognition(request)?;

    Ok((StatusCode::CREATED, Json(cognition)))
}
