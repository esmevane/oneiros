use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<SensationResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_sensation(SensationRequests::ListSensations(ListSensationsRequest))?;

    Ok(Json(response))
}
