use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<LevelName>,
) -> Result<Json<LevelResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_level(LevelRequests::GetLevel(GetLevelRequest { name }))?;

    Ok(Json(response))
}
