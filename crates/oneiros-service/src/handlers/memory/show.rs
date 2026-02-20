use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Identity<MemoryId, Memory>>>, Error> {
    let by_id = ticket.db.get_memory(&identifier)?;

    let memory = if let Some(m) = by_id {
        m
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_memory_by_link(link.to_string())?
            .ok_or(NotFound::Memory(identifier))?
    } else {
        return Err(NotFound::Memory(identifier).into());
    };

    let record = Record::new(memory)?;
    Ok(Json(record))
}
