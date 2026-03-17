use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
    routing,
};
use oneiros_model::*;

use oneiros_resource::{Feature, Server};

use crate::resource_agent::Agent;
use crate::{ServiceState, ServiceStateError};

// ── Error handling ──────────────────────────────────────────────────

impl IntoResponse for ServiceStateError {
    fn into_response(self) -> AxumResponse {
        let (status, message) = match &self {
            ServiceStateError::Scope(scope_err) => {
                use crate::ProjectScopeError::*;
                match scope_err {
                    NotFound(e) => (StatusCode::NOT_FOUND, e.to_string()),
                    Conflict(e) => (StatusCode::CONFLICT, e.to_string()),
                    Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
                    Effects(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
                }
            }
            ServiceStateError::DatabasePoisoned => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        let body = serde_json::json!({ "error": message });
        (status, Json(body)).into_response()
    }
}

// ── Feature<Server> for Agent ───────────────────────────────────────
//
// The Agent resource opts into the HTTP layer. Its surface is an axum
// Router parameterized by ServiceState — the scope binding happens
// through axum's state extraction, not through our trait.

impl Feature<Server> for Agent {
    type Surface = Router<ServiceState>;

    fn feature(&self) -> Self::Surface {
        Router::new()
            .route("/", routing::post(create).get(list))
            .route("/{name}", routing::get(show).put(update).delete(remove))
    }
}

// Keep the static method as a convenience — delegates to the trait.
impl Agent {
    pub fn http_router() -> Router<ServiceState> {
        <Agent as Feature<Server>>::feature(&Agent)
    }
}

// ── Handlers ────────────────────────────────────────────────────────
//
// Each handler: extract args, fulfill, format response.
// The state.fulfill() call is synchronous — no MutexGuard crosses
// an await boundary.

async fn create(
    State(state): State<ServiceState>,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<AgentResponses>), ServiceStateError> {
    let response = state.fulfill::<Agent>(AgentRequests::CreateAgent(request))?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(state): State<ServiceState>,
) -> Result<Json<AgentResponses>, ServiceStateError> {
    let response = state.fulfill::<Agent>(AgentRequests::ListAgents(ListAgentsRequest))?;
    Ok(Json(response))
}

async fn show(
    State(state): State<ServiceState>,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponses>, ServiceStateError> {
    let response = state.fulfill::<Agent>(AgentRequests::GetAgent(GetAgentRequest { name }))?;
    Ok(Json(response))
}

async fn update(
    State(state): State<ServiceState>,
    Path(name): Path<AgentName>,
    Json(mut request): Json<UpdateAgentRequest>,
) -> Result<Json<AgentResponses>, ServiceStateError> {
    request.name = name;
    let response = state.fulfill::<Agent>(AgentRequests::UpdateAgent(request))?;
    Ok(Json(response))
}

async fn remove(
    State(state): State<ServiceState>,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponses>, ServiceStateError> {
    let response = state.fulfill::<Agent>(AgentRequests::RemoveAgent(RemoveAgentRequest { name }))?;
    Ok(Json(response))
}
