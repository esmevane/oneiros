use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(sensation): Json<Sensation>,
) -> Result<Json<SensationResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_sensation(SensationRequests::SetSensation(sensation))?;

    Ok(Json(response))
}
