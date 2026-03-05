use axum::{Json, extract::Path};
use oneiros_model::{SensationName, SensationResponses};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<SensationName>,
) -> Result<Json<SensationResponses>, Error> {
    let sensation = ticket.service().get_sensation(&given_name)?;

    Ok(Json(sensation))
}
