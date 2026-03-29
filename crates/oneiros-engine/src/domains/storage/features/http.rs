use axum::{Json, Router, extract::Path, http::StatusCode, routing};
use serde::Deserialize;

use crate::*;

pub struct StorageRouter;

impl StorageRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/storage",
            Router::new()
                .route("/", routing::get(list).post(upload))
                .route("/{ref_key}", routing::get(show).delete(remove)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct UploadBody {
    key: StorageKey,
    #[serde(default)]
    description: Description,
    /// Base64-encoded binary data for JSON transport.
    data: Vec<u8>,
}

async fn upload(
    context: ProjectContext,
    Json(body): Json<UploadBody>,
) -> Result<(StatusCode, Json<StorageResponse>), StorageError> {
    let response = StorageService::upload(&context, body.key, body.description, body.data).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(context: ProjectContext) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::list(&context)?))
}

async fn show(
    context: ProjectContext,
    Path(ref_key): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    let storage_ref = StorageRef(ref_key);
    let key = storage_ref.decode().map_err(|_| StorageError::InvalidRef)?;
    Ok(Json(StorageService::show(&context, &key)?))
}

async fn remove(
    context: ProjectContext,
    Path(ref_key): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    let storage_ref = StorageRef(ref_key);
    let key = storage_ref.decode().map_err(|_| StorageError::InvalidRef)?;
    Ok(Json(StorageService::remove(&context, &key).await?))
}
