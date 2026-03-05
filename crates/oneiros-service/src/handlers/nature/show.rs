use axum::{Json, extract::Path};
use oneiros_model::{NatureName, NatureResponses};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<NatureName>,
) -> Result<Json<NatureResponses>, Error> {
    let nature = ticket.service().get_nature(&given_name)?;

    Ok(Json(nature))
}
