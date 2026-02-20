use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Identity<CognitionId, Cognition>>>, Error> {
    let by_id = ticket.db.get_cognition(&identifier)?;

    let cognition = if let Some(c) = by_id {
        c
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_cognition_by_link(link.to_string())?
            .ok_or(NotFound::Cognition(identifier))?
    } else {
        return Err(NotFound::Cognition(identifier).into());
    };

    let record = Record::new(cognition)?;
    Ok(Json(record))
}
