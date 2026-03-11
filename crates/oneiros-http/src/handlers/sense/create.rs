use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(SenseRequests::Sense(SenseRequest { agent }))?,
    ))
}
