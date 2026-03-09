use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<IntrospectingResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_introspect(IntrospectingRequests::Introspect(IntrospectRequest {
            agent,
        }))?;

    Ok(Json(response))
}
