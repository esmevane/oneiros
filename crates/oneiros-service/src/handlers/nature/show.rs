use axum::{Json, extract::Path};
use oneiros_model::{NatureName, NatureRecord};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<NatureName>,
) -> Result<Json<NatureRecord>, Error> {
    let nature = ticket
        .db
        .get_nature(&given_name)?
        .ok_or(NotFound::Nature(given_name))?;

    Ok(Json(nature))
}
