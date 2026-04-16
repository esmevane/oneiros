use aide::axum::{ApiRouter, routing};
use axum::{Json, extract::Query};

use crate::*;

pub struct SearchRouter;

impl SearchRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/search",
            ApiRouter::new().api_route(
                "/",
                routing::get_with(search, |op| {
                    resource_op!(op, SearchDocs::Search).security_requirement("BearerToken")
                }),
            ),
        )
    }
}

async fn search(
    context: ProjectContext,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, SearchError> {
    Ok(Json(SearchService::search(&context, &params).await?))
}
