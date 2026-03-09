use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<LevelResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_level(LevelRequests::ListLevels(ListLevelsRequest))?;

    Ok(Json(response))
}
