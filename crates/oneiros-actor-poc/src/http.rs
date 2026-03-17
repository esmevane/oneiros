//! HTTP surface — produces an axum Router from the registry.
//!
//! Handlers dispatch through actor handles. The registry is the axum state.
//! No Mutex, no pollster_block_on — the Handle::send is natively async.

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
    routing,
};
use oneiros_model::*;

use crate::agent::AgentError;
use crate::registry::Registry;

// ── Error mapping ───────────────────────────────────────────────────

impl IntoResponse for AgentError {
    fn into_response(self) -> AxumResponse {
        let (status, message) = match &self {
            AgentError::NotFound(e) => (StatusCode::NOT_FOUND, e.to_string()),
            AgentError::Conflict(e) => (StatusCode::CONFLICT, e.to_string()),
        };

        let body = serde_json::json!({ "error": message });
        (status, Json(body)).into_response()
    }
}

// ── Router construction ─────────────────────────────────────────────

/// Build the HTTP router from the registry.
///
/// The registry IS the axum state — handlers extract it and use
/// actor handles directly. No intermediate ServiceState needed.
pub fn http_router(registry: Registry) -> Router {
    Router::new()
        .nest("/agents", agent_routes())
        .with_state(registry)
}

fn agent_routes() -> Router<Registry> {
    Router::new()
        .route("/", routing::post(create_agent).get(list_agents))
        .route(
            "/{name}",
            routing::get(get_agent).put(update_agent).delete(remove_agent),
        )
}

// ── Handlers ────────────────────────────────────────────────────────
//
// Each handler: extract args, send message to actor, format response.
// The actor handle's .send() is natively async — no blocking bridge needed.

async fn create_agent(
    State(registry): State<Registry>,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<AgentResponses>), AgentError> {
    let response = registry
        .agents
        .send(AgentRequests::CreateAgent(request))
        .await
        .expect("agent actor alive")?;

    Ok((StatusCode::CREATED, Json(response)))
}

async fn list_agents(
    State(registry): State<Registry>,
) -> Result<Json<AgentResponses>, AgentError> {
    let response = registry
        .agents
        .send(AgentRequests::ListAgents(ListAgentsRequest))
        .await
        .expect("agent actor alive")?;

    Ok(Json(response))
}

async fn get_agent(
    State(registry): State<Registry>,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponses>, AgentError> {
    let response = registry
        .agents
        .send(AgentRequests::GetAgent(GetAgentRequest { name }))
        .await
        .expect("agent actor alive")?;

    Ok(Json(response))
}

async fn update_agent(
    State(registry): State<Registry>,
    Path(name): Path<AgentName>,
    Json(mut request): Json<UpdateAgentRequest>,
) -> Result<Json<AgentResponses>, AgentError> {
    request.name = name;
    let response = registry
        .agents
        .send(AgentRequests::UpdateAgent(request))
        .await
        .expect("agent actor alive")?;

    Ok(Json(response))
}

async fn remove_agent(
    State(registry): State<Registry>,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponses>, AgentError> {
    let response = registry
        .agents
        .send(AgentRequests::RemoveAgent(RemoveAgentRequest { name }))
        .await
        .expect("agent actor alive")?;

    Ok(Json(response))
}
