use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct MemoryRouter;

impl MemoryRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/memories",
            Router::new()
                .route("/", routing::get(list).post(add))
                .route("/{id}", routing::get(show)),
        )
    }
}

async fn add(
    context: ProjectContext,
    Json(body): Json<AddMemory>,
) -> Result<(StatusCode, Json<MemoryResponse>), MemoryError> {
    let response = MemoryService::add(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListMemories>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(MemoryService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(id): Path<MemoryId>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(
        MemoryService::get(&context, &GetMemory::builder().id(id).build()).await?,
    ))
}
