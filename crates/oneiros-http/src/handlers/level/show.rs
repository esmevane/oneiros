use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<LevelName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(LevelRequests::GetLevel(
        GetLevelRequest { name },
    ))?))
}
