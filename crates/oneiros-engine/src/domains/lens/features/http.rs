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
                .route("/explain", post(explain))
                .route("/query", post(query)),
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

async fn query(
    axum::extract::State(state): axum::extract::State<ServerState>,
    scope: Scope<AtBookmark>,
    Json(body): Json<QueryLens>,
) -> Result<Json<LensResponse>, LensError> {
    Ok(Json(
        LensService::query(&scope, state.canons(), &body).await?,
    ))
}
