use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<CognitionId>,
) -> Result<Json<Cognition>, Error> {
    let cognition = ticket.service().get_cognition(&id)?;

    Ok(Json(cognition))
}
