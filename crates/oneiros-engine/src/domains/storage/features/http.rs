use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::contexts::ProjectContext;

use super::super::errors::StorageError;
use super::super::responses::StorageResponse;
use super::super::service::StorageService;

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list).post(upload))
        .route("/{id}", routing::get(show).delete(remove))
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
    let response = StorageService::upload(&ctx, body.name, body.content_type, body.data)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::get(&ctx, &id)?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::remove(&ctx, &id)?))
}
