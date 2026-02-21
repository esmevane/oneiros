use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<ConnectionId, ConnectionLink>>,
) -> Result<Json<Record<Identity<ConnectionId, Connection>>>, Error> {
    let connection = ticket
        .db
        .get_connection_by_key(&key)?
        .ok_or(NotFound::Connection(key))?;

    let record = Record::new(connection)?;
    Ok(Json(record))
}
