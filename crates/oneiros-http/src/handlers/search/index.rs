use axum::{Json, extract::Query};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(request): Query<SearchRequest>,
) -> Result<Json<SearchResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_search(SearchRequests::Search(request))?;

    Ok(Json(response))
}
