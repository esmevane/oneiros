use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<NatureName, NatureLink>>,
) -> Result<Json<Record<Nature>>, Error> {
    let nature = ticket
        .db
        .get_nature_by_key(&key)?
        .ok_or(NotFound::Nature(key))?;

    let record = Record::new(nature)?;
    Ok(Json(record))
}
