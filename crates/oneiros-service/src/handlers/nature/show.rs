use axum::{Json, extract::Path};
use oneiros_model::{Nature, NatureName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<NatureName>,
) -> Result<Json<Nature>, Error> {
    let nature = ticket.service().get_nature(&given_name)?;

    Ok(Json(nature))
}
