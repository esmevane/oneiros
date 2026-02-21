use axum::{Json, http::StatusCode};
use chrono::Utc;
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateConnectionRequest>,
) -> Result<(StatusCode, Json<Identity<ConnectionId, Connection>>), Error> {
    // Validate that the referenced nature exists.
    ticket
        .db
        .get_nature(&request.nature)?
        .ok_or(NotFound::Nature(Key::Id(request.nature.clone())))?;

    let connection = Identity::new(
        ConnectionId::new(),
        Connection {
            nature: request.nature,
            from_link: request.from_link,
            to_link: request.to_link,
            created_at: Utc::now(),
        },
    );

    let event = Events::Connection(ConnectionEvents::ConnectionCreated(connection.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(connection)))
}
