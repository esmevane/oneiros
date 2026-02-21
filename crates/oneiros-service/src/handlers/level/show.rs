use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<LevelName, LevelLink>>,
) -> Result<Json<Record<Level>>, Error> {
    let level = ticket
        .db
        .get_level_by_key(&key)?
        .ok_or(NotFound::Level(key))?;

    let record = Record::new(level)?;
    Ok(Json(record))
}
