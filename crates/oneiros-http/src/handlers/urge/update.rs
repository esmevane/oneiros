use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(urge): Json<Urge>,
) -> Result<Json<UrgeResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_urge(UrgeRequests::SetUrge(urge))?;

    Ok(Json(response))
}
