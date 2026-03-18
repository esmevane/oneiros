use axum::{
    Json, Router,
    extract::{Query, State},
    routing,
};
use serde::Deserialize;

use crate::contexts::ProjectContext;

use super::super::errors::SearchError;
use super::super::responses::SearchResponse;
use super::super::service::SearchService;

pub const PATH: &str = "/search";

pub fn routes() -> Router<ProjectContext> {
    Router::new().route("/", routing::get(search))
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String,
    agent: Option<String>,
}

async fn search(
    State(ctx): State<ProjectContext>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, SearchError> {
    Ok(Json(SearchService::search(
        &ctx,
        &params.q,
        params.agent.as_deref(),
    )?))
}
