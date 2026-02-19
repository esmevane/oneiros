use axum::{Json, extract::Path};
use oneiros_model::{Connection, ConnectionId, Identity};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<Identity<ConnectionId, Connection>>, Error> {
    let connection = ticket
        .db
        .get_connection(id.to_string())?
        .ok_or(NotFound::Connection(id))?;

    Ok(Json(connection))
}
