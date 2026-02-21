use axum::{Json, extract::Path};
use oneiros_model::{SensationName, SensationRecord};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<SensationName>,
) -> Result<Json<SensationRecord>, Error> {
    let sensation = ticket
        .db
        .get_sensation(&given_name)?
        .ok_or(NotFound::Sensation(given_name))?;

    Ok(Json(sensation))
}
