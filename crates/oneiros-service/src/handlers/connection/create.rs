use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateConnectionRequest>,
) -> Result<(StatusCode, Json<Record<ConnectionId, Connection>>), Error> {
    // Validate that the referenced nature exists.
    ticket
        .db
        .get_nature(&request.nature)?
        .ok_or(NotFound::Nature(request.nature.clone()))?;

    let connection = Connection {
        nature: request.nature,
        from_link: request.from_link,
        to_link: request.to_link,
    };

    let record = Record::create(connection);

    let event = Events::Connection(ConnectionEvents::ConnectionCreated(record.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::CREATED, Json(record)))
}
