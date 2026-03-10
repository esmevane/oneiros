use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<UrgeName>,
) -> Result<StatusCode, Error> {
    ticket
        .service()
        .dispatch_urge(UrgeRequests::RemoveUrge(RemoveUrgeRequest { name }))?;

    Ok(StatusCode::OK)
}
