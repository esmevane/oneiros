use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<NatureName>,
) -> Result<Json<NatureResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_nature(NatureRequests::GetNature(GetNatureRequest { name }))?;

    Ok(Json(response))
}
