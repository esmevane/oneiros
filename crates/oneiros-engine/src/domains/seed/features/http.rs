use axum::{Json, Router, http::StatusCode, routing};

use crate::*;

pub struct SeedRouter;

impl SeedRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/seed",
            Router::new()
                .route("/core", routing::post(seed_core))
                .route("/agents", routing::post(seed_agents)),
        )
    }
}

async fn seed_core(context: ProjectContext) -> Result<(StatusCode, Json<SeedResponse>), SeedError> {
    let response = SeedService::core(&context).await?;
    Ok((StatusCode::OK, Json(response)))
}

async fn seed_agents(
    context: ProjectContext,
) -> Result<(StatusCode, Json<SeedResponse>), SeedError> {
    let response = SeedService::agents(&context).await?;
    Ok((StatusCode::OK, Json(response)))
}
