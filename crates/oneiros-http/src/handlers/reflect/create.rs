use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<ReflectingResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_reflect(ReflectingRequests::Reflect(ReflectRequest { agent }))?;

    Ok(Json(response))
}
