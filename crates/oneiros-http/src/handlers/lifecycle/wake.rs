use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<LifecycleResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_lifecycle(LifecycleRequests::Wake(WakeRequest { agent }))?;

    Ok(Json(response))
}
