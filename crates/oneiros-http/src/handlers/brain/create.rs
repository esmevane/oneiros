use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use oneiros_model::*;
use std::sync::Arc;

use crate::*;

pub(crate) async fn handler(
    State(state): State<Arc<ServiceState>>,
    Json(request): Json<CreateBrainRequest>,
) -> Result<(StatusCode, Json<BrainResponses>), Error> {
    let response = state
        .system_service()?
        .dispatch_brain(BrainRequests::CreateBrain(request))?;

    Ok((StatusCode::CREATED, Json(response)))
}
