use axum::{Json, Router, extract::Path, http::StatusCode, routing};
use oneiros_model::*;
use oneiros_service::OneirosService;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::post(create))
        .route("/", routing::get(index))
        .route("/{name}", routing::get(show))
        .route("/{name}", routing::put(update))
        .route("/{name}", routing::delete(delete))
}

async fn create(
    ticket: OneirosContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    Ok((
        StatusCode::CREATED,
        Json(ticket.dispatch(AgentRequests::CreateAgent(request))?),
    ))
}

async fn delete(
    ticket: OneirosContext,
    Path(name): Path<AgentName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(AgentRequests::RemoveAgent(
        RemoveAgentRequest { name },
    ))?))
}

async fn index(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(AgentRequests::ListAgents(ListAgentsRequest))?,
    ))
}

async fn show(
    ticket: OneirosContext,
    Path(name): Path<AgentName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(AgentRequests::GetAgent(
        GetAgentRequest { name },
    ))?))
}

async fn update(
    ticket: OneirosContext,
    Path(name): Path<AgentName>,
    Json(mut request): Json<UpdateAgentRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    request.name = name;

    Ok((
        StatusCode::OK,
        Json(ticket.dispatch(AgentRequests::UpdateAgent(request))?),
    ))
}
