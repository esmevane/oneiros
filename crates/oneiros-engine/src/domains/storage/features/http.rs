use axum::{Json, Router, extract::Path, http::StatusCode, routing};

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

async fn upload(
    context: ProjectContext,
    Json(body): Json<UploadStorage>,
) -> Result<(StatusCode, Json<StorageResponse>), StorageError> {
    let response = StorageService::upload(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(context: ProjectContext) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::list(&context).await?))
}

async fn show(
    context: ProjectContext,
    Path(ref_key): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    let storage_ref = StorageRef(ref_key);
    let key = storage_ref.decode().map_err(|_| StorageError::InvalidRef)?;
    Ok(Json(
        StorageService::show(&context, &GetStorage::builder().key(key).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(ref_key): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    let storage_ref = StorageRef(ref_key);
    let key = storage_ref.decode().map_err(|_| StorageError::InvalidRef)?;
    Ok(Json(
        StorageService::remove(&context, &RemoveStorage::builder().key(key).build()).await?,
    ))
}
