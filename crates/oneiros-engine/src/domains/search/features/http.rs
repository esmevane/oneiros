use axum::{
    Json, Router,
    extract::{Query, State},
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct SearchRouter;

impl SearchRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest("/search", Router::new().route("/", routing::get(search)))
    }
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
    let agent_name = params.agent.as_deref().map(AgentName::new);
    Ok(Json(SearchService::search(
        &ctx,
        &params.q,
        agent_name.as_ref(),
    )?))
}
