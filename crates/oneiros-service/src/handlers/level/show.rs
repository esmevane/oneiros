use axum::{Json, extract::Path};
use oneiros_model::{Level, LevelName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<LevelName>,
) -> Result<Json<Level>, Error> {
    let level = ticket
        .db
        .get_level(&given_name)?
        .ok_or(NotFound::Level(given_name))?;

    Ok(Json(level))
}
