use axum::{Json, extract::Query};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(request): Query<ListCognitionsRequest>,
) -> Result<Json<CognitionResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_cognition(CognitionRequests::ListCognitions(request))?;

    Ok(Json(response))
}
