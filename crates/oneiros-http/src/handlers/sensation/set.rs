use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(sensation): Json<Sensation>,
) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(SensationRequests::SetSensation(sensation))?,
    ))
}
