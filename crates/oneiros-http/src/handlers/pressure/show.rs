use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<PressureResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_pressure(PressureRequests::GetPressure(GetPressureRequest { agent }))?;

    Ok(Json(response))
}
