use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<SenseResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_sense(SenseRequests::Sense(SenseRequest { agent }))?;

    Ok(Json(response))
}
