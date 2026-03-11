use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<PressureResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_pressure(PressureRequests::ListPressures(ListPressuresRequest))?;

    Ok(Json(response))
}
