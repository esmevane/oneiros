use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(nature): Json<Nature>,
) -> Result<Json<NatureResponses>, Error> {
    let nature = ticket.service().set_nature(nature)?;

    Ok(Json(nature))
}
