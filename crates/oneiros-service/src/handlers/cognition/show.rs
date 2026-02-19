use axum::{Json, extract::Path};
use oneiros_model::{Cognition, CognitionId, Identity};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<CognitionId>,
) -> Result<Json<Identity<CognitionId, Cognition>>, Error> {
    let cognition = ticket
        .db
        .get_cognition(id.to_string())?
        .ok_or(NotFound::Cognition(id))?;

    Ok(Json(cognition))
}
