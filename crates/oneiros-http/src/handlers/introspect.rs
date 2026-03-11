use axum::{Json, Router, extract::Path, routing};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new().route("/{agent_name}", routing::post(create))
}

async fn create(
    ticket: OneirosContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(IntrospectingRequests::Introspect(
        IntrospectRequest { agent },
    ))?))
}
