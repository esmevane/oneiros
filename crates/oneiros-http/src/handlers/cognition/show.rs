use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<CognitionId>,
) -> Result<Json<CognitionResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_cognition(CognitionRequests::GetCognition(GetCognitionRequest { id }))?;

    Ok(Json(response))
}
