use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(sensation): Json<Sensation>,
) -> Result<Json<SensationResponses>, Error> {
    let sensation = ticket.service().set_sensation(sensation)?;

    Ok(Json(sensation))
}
