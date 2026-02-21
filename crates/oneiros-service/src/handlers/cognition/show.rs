use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<CognitionId>,
) -> Result<Json<Record<CognitionId, Cognition>>, Error> {
    let cognition = ticket
        .db
        .get_cognition(id.to_string())?
        .ok_or(NotFound::Cognition(id))?;

    Ok(Json(cognition))
}
