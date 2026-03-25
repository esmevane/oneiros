use axum::{Json, Router, extract::Query, routing};
use serde::Deserialize;

use crate::*;

pub struct SearchRouter;

impl SearchRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest("/search", Router::new().route("/", routing::get(search)))
    }
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String,
    agent: Option<AgentName>,
}

async fn search(
    context: ProjectContext,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, SearchError> {
    Ok(Json(SearchService::search(
        &context,
        &params.q,
        params.agent.as_ref(),
    )?))
}
