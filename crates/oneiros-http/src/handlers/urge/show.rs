use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<UrgeName>,
) -> Result<Json<UrgeResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_urge(UrgeRequests::GetUrge(GetUrgeRequest { name }))?;

    Ok(Json(response))
}
