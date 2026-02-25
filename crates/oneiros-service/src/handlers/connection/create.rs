use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateConnectionRequest>,
) -> Result<(StatusCode, Json<Connection>), Error> {
    // Validate that the referenced nature exists.
    ticket
        .db
        .get_nature(&request.nature)?
        .ok_or(NotFound::Nature(request.nature.clone()))?;

    let connection = Connection::create(request.nature, request.from_link, request.to_link);

    let event = Events::Connection(ConnectionEvents::ConnectionCreated(connection.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::CREATED, Json(connection)))
}
