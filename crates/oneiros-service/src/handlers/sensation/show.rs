use axum::{Json, extract::Path};
use oneiros_model::{Sensation, SensationName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<SensationName>,
) -> Result<Json<Sensation>, Error> {
    let sensation = ticket.service().get_sensation(&given_name)?;

    Ok(Json(sensation))
}
