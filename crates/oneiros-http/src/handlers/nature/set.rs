use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(nature): Json<Nature>,
) -> Result<Json<NatureResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_nature(NatureRequests::SetNature(nature))?;

    Ok(Json(response))
}
