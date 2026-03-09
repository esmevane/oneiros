use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<SensationName>,
) -> Result<Json<SensationResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_sensation(SensationRequests::GetSensation(GetSensationRequest {
            name,
        }))?;

    Ok(Json(response))
}
