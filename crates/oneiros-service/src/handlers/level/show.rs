use axum::{Json, extract::Path};
use oneiros_model::{LevelName, LevelRecord};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<LevelName>,
) -> Result<Json<LevelRecord>, Error> {
    let level = ticket
        .db
        .get_level(&given_name)?
        .ok_or(NotFound::Level(given_name))?;

    Ok(Json(level))
}
