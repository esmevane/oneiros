use axum::{Json, extract::Query};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct SearchParams {
    pub q: String,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<SearchParams>,
) -> Result<Json<SearchResults>, Error> {
    let results = ticket.db.search_expressions(&params.q)?;

    Ok(Json(SearchResults {
        query: params.q,
        results,
    }))
}
