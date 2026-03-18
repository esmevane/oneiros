//! Agent HTTP driving adapter — translates HTTP into domain service calls.

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
    routing,
};
use oneiros_model::*;

use super::{AgentError, AgentService};
use crate::ports::AppContext;

impl IntoResponse for AgentError {
    fn into_response(self) -> AxumResponse {
        let (status, message) = match &self {
            AgentError::NotFound(e) => (StatusCode::NOT_FOUND, e.to_string()),
            AgentError::Conflict(e) => (StatusCode::CONFLICT, e.to_string()),
            AgentError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        let body = serde_json::json!({ "error": message });
        (status, Json(body)).into_response()
    }
}

/// Agent HTTP routes — the driving adapter's registration.
pub fn routes() -> Router<AppContext> {
    Router::new()
        .route("/", routing::post(create).get(list))
        .route("/{name}", routing::get(show).put(update).delete(remove))
}

async fn create(
    State(ctx): State<AppContext>,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<AgentResponses>), AgentError> {
    let response = AgentService::create(&ctx, request)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(State(ctx): State<AppContext>) -> Result<Json<AgentResponses>, AgentError> {
    Ok(Json(AgentService::list(&ctx)?))
}

async fn show(
    State(ctx): State<AppContext>,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponses>, AgentError> {
    Ok(Json(AgentService::get(&ctx, &name)?))
}

async fn update(
    State(ctx): State<AppContext>,
    Path(name): Path<AgentName>,
    Json(mut request): Json<UpdateAgentRequest>,
) -> Result<Json<AgentResponses>, AgentError> {
    request.name = name;
    Ok(Json(AgentService::update(&ctx, request)?))
}

async fn remove(
    State(ctx): State<AppContext>,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponses>, AgentError> {
    Ok(Json(AgentService::remove(&ctx, name)?))
}
