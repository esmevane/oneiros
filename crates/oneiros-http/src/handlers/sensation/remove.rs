use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<SensationName>,
) -> Result<StatusCode, Error> {
    ticket
        .service()
        .dispatch_sensation(SensationRequests::RemoveSensation(RemoveSensationRequest {
            name,
        }))?;

    Ok(StatusCode::OK)
}
