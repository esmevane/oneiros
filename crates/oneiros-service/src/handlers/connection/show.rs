use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<Record<ConnectionId, Connection>>, Error> {
    let connection = ticket
        .db
        .get_connection(id.to_string())?
        .ok_or(NotFound::Connection(id))?;

    Ok(Json(connection))
}
