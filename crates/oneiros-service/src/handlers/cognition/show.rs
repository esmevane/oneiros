use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<CognitionId, CognitionLink>>,
) -> Result<Json<Record<Identity<CognitionId, Cognition>>>, Error> {
    let cognition = ticket
        .db
        .get_cognition_by_key(&key)?
        .ok_or(NotFound::Cognition(key))?;

    let record = Record::new(cognition)?;
    Ok(Json(record))
}
