use axum::{Json, extract::Path};
use oneiros_model::{LevelName, LevelResponses};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<LevelName>,
) -> Result<Json<LevelResponses>, Error> {
    let level = ticket.service().get_level(&given_name)?;

    Ok(Json(level))
}
