use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct BrainRouter;

impl BrainRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/brains",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{name}", routing::get(show)),
        )
    }
}

async fn create(
    context: SystemContext,
    Json(body): Json<CreateBrain>,
) -> Result<(StatusCode, Json<BrainResponse>), BrainError> {
    let response = BrainService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: SystemContext,
    Query(params): Query<ListBrains>,
) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(BrainService::list(&context, &params).await?))
}

async fn show(
    context: SystemContext,
    Path(name): Path<String>,
) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(
        BrainService::get(&context, &GetBrain::builder().name(name).build()).await?,
    ))
}
