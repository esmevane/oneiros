use aide::axum::ApiRouter;
use axum::Json;
use axum::routing::post;

use crate::*;

pub(crate) struct LensRouter;

impl LensRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/lens",
            ApiRouter::new()
                .route("/parse", post(parse))
                .route("/explain", post(explain)),
        )
    }
}

async fn parse(Json(body): Json<ParseLens>) -> Result<Json<LensResponse>, LensError> {
    Ok(Json(LensService::parse(&body)?))
}

async fn explain(
    scope: Scope<AtBookmark>,
    Json(body): Json<ExplainLens>,
) -> Result<Json<LensResponse>, LensError> {
    Ok(Json(LensService::explain(&scope, &body).await?))
}
