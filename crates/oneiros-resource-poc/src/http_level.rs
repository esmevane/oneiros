use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};
use oneiros_model::*;

use crate::resource_level::Level;
use crate::{ServiceState, ServiceStateError};

// ── Level HTTP resource ─────────────────────────────────────────────

impl Level {
    /// The Level resource's HTTP router.
    ///
    /// Vocabulary CRUD: set (PUT), get, list, remove.
    /// No POST — "set" is idempotent, so PUT is the right verb.
    pub fn http_router() -> Router<ServiceState> {
        Router::new()
            .route("/", routing::get(list))
            .route("/{name}", routing::put(set).get(show).delete(remove))
    }
}

// ── Handlers ────────────────────────────────────────────────────────

async fn set(
    State(state): State<ServiceState>,
    Path(name): Path<LevelName>,
    Json(mut level): Json<oneiros_model::Level>,
) -> Result<(StatusCode, Json<LevelResponses>), ServiceStateError> {
    level.name = name;
    let response = state.fulfill::<Level>(LevelRequests::SetLevel(level))?;
    Ok((StatusCode::OK, Json(response)))
}

async fn list(
    State(state): State<ServiceState>,
) -> Result<Json<LevelResponses>, ServiceStateError> {
    let response = state.fulfill::<Level>(LevelRequests::ListLevels(ListLevelsRequest))?;
    Ok(Json(response))
}

async fn show(
    State(state): State<ServiceState>,
    Path(name): Path<LevelName>,
) -> Result<Json<LevelResponses>, ServiceStateError> {
    let response = state.fulfill::<Level>(LevelRequests::GetLevel(GetLevelRequest { name }))?;
    Ok(Json(response))
}

async fn remove(
    State(state): State<ServiceState>,
    Path(name): Path<LevelName>,
) -> Result<Json<LevelResponses>, ServiceStateError> {
    let response =
        state.fulfill::<Level>(LevelRequests::RemoveLevel(RemoveLevelRequest { name }))?;
    Ok(Json(response))
}
