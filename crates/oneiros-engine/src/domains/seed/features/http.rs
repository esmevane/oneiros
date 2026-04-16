use aide::axum::{ApiRouter, routing};
use axum::{Json, http::StatusCode};

use crate::*;

pub struct SeedRouter;

impl SeedRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/seed",
            ApiRouter::new()
                .api_route(
                    "/core",
                    routing::post_with(seed_core, |op| {
                        resource_op!(op, SeedDocs::SeedCore)
                            .security_requirement("BearerToken")
                            .response::<200, Json<SeedResponse>>()
                    }),
                )
                .api_route(
                    "/agents",
                    routing::post_with(seed_agents, |op| {
                        resource_op!(op, SeedDocs::SeedAgents)
                            .security_requirement("BearerToken")
                            .response::<200, Json<SeedResponse>>()
                    }),
                ),
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
