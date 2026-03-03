use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use oneiros_model::*;
use std::sync::Arc;

use crate::*;

pub(crate) async fn handler(
    State(state): State<Arc<ServiceState>>,
    Json(request): Json<CreateBrainRequest>,
) -> Result<(StatusCode, Json<BrainInfo>), Error> {
    let info = state.system_service()?.create_brain(request)?;

    Ok((StatusCode::CREATED, Json(info)))
}
