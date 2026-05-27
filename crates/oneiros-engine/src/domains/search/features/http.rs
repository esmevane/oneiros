use aide::axum::{ApiRouter, routing};
use axum::{Json, extract::Query};

use crate::*;

pub(crate) struct SearchRouter;

impl SearchRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
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

#[expect(deprecated)]
async fn search(
    context: ProjectLog,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, SearchError> {
    Ok(Json(SearchService::search(&context, &params).await?))
}
