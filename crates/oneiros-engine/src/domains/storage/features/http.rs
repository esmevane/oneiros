use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct StorageRouter;

impl StorageRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/storage",
            Router::new()
                .route("/", routing::get(list).post(upload))
                .route("/{id}", routing::get(show).delete(remove)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct UploadBody {
    name: String,
    content_type: String,
    /// Base64-encoded binary data for JSON transport.
    data: Vec<u8>,
}

async fn upload(
    State(ctx): State<ProjectContext>,
    Json(body): Json<UploadBody>,
) -> Result<(StatusCode, Json<StorageResponse>), StorageError> {
    let response = StorageService::upload(
        &ctx,
        StorageName::new(body.name),
        Label::new(body.content_type),
        body.data,
    )?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(id): Path<StorageId>,
) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::get(&ctx, &id)?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(id): Path<StorageId>,
) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::remove(&ctx, &id)?))
}
