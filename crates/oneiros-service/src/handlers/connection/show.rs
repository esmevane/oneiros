use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Identity<ConnectionId, Connection>>>, Error> {
    let by_id = ticket.db.get_connection(&identifier)?;

    let connection = if let Some(c) = by_id {
        c
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_connection_by_link(link.to_string())?
            .ok_or(NotFound::Connection(identifier))?
    } else {
        return Err(NotFound::Connection(identifier).into());
    };

    let record = Record::new(connection)?;
    Ok(Json(record))
}
