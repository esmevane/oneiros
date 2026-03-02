use axum::extract::Path;
use axum::http::StatusCode;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ConnectionId>,
) -> Result<StatusCode, Error> {
    ticket
        .db
        .get_connection(id.to_string())?
        .ok_or(NotFound::Connection(id))?;

    let event = Events::Connection(ConnectionEvents::ConnectionRemoved { id });

    ticket.db.log_event(&event, projections::BRAIN)?;
    ticket.broadcast(&event);

    Ok(StatusCode::NO_CONTENT)
}
