use axum::extract::Path;
use axum::http::StatusCode;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ConnectionId>,
) -> Result<StatusCode, Error> {
    ticket.service().remove_connection(&id)?;

    Ok(StatusCode::NO_CONTENT)
}
