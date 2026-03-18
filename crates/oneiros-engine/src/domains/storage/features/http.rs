use std::path::PathBuf;

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

/// State for storage routes — wraps the project context with the blob data directory.
#[derive(Clone)]
pub struct StorageState {
    pub ctx: ProjectContext,
    pub data_dir: PathBuf,
}

pub fn routes() -> Router<StorageState> {
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
    State(state): State<StorageState>,
    Json(body): Json<UploadBody>,
) -> Result<(StatusCode, Json<StorageResponse>), StorageError> {
    let response = StorageService::upload(
        &state.ctx,
        &state.data_dir,
        body.name,
        body.content_type,
        body.data,
    )?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(state): State<StorageState>,
) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::list(&state.ctx)?))
}

async fn show(
    State(state): State<StorageState>,
    Path(id): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::get(&state.ctx, &id)?))
}

async fn remove(
    State(state): State<StorageState>,
    Path(id): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::remove(&state.ctx, &state.data_dir, &id)?))
}
