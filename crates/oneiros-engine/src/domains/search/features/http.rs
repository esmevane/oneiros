use axum::{Json, Router, extract::Query, routing};

use crate::*;

pub struct SearchRouter;

impl SearchRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest("/search", Router::new().route("/", routing::get(search)))
    }
}

async fn search(
    context: ProjectContext,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, SearchError> {
    Ok(Json(SearchService::search(&context, &params).await?))
}
