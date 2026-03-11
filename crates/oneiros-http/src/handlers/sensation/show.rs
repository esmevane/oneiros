use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<SensationName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(SensationRequests::GetSensation(
        GetSensationRequest { name },
    ))?))
}
