use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<NatureName>,
) -> Result<StatusCode, Error> {
    ticket
        .service()
        .dispatch_nature(NatureRequests::RemoveNature(RemoveNatureRequest { name }))?;

    Ok(StatusCode::OK)
}
