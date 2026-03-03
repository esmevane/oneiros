use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<Connection>, Error> {
    let connection = ticket.service().get_connection(&id)?;

    Ok(Json(connection))
}
