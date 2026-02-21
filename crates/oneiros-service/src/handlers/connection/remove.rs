use axum::extract::Path;
use axum::http::StatusCode;
use oneiros_model::{ConnectionId, Key};
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ConnectionId>,
) -> Result<StatusCode, Error> {
    ticket
        .db
        .get_connection(&id)?
        .ok_or(NotFound::Connection(Key::Id(id.clone())))?;

    let event = Events::Connection(ConnectionEvents::ConnectionRemoved { id });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok(StatusCode::NO_CONTENT)
}
