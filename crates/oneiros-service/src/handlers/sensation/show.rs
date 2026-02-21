use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<SensationName, SensationLink>>,
) -> Result<Json<Record<Sensation>>, Error> {
    let sensation = ticket
        .db
        .get_sensation_by_key(&key)?
        .ok_or(NotFound::Sensation(key))?;

    let record = Record::new(sensation)?;
    Ok(Json(record))
}
