use axum::{Json, Router, extract::Path, http::StatusCode, routing};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/wake/{agent_name}", routing::post(wake))
        .route("/sleep/{agent_name}", routing::post(sleep))
        .route("/emerge", routing::post(emerge))
        .route("/recede/{agent_name}", routing::post(recede))
}

async fn wake(
    ticket: OneirosContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(LifecycleRequests::Wake(
        WakeRequest { agent },
    ))?))
}

async fn sleep(
    ticket: OneirosContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(LifecycleRequests::Sleep(
        SleepRequest { agent },
    ))?))
}

async fn emerge(
    ticket: OneirosContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    Ok((
        StatusCode::CREATED,
        Json(ticket.dispatch(LifecycleRequests::Emerge(request))?),
    ))
}

async fn recede(
    ticket: OneirosContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(LifecycleRequests::Recede(
        RecedeRequest { agent },
    ))?))
}
