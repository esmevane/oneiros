use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<UrgeResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_urge(UrgeRequests::ListUrges(ListUrgesRequest))?;

    Ok(Json(response))
}
