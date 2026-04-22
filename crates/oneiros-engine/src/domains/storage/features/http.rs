use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct StorageRouter;

impl StorageRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/storage",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, StorageDocs::List).security_requirement("BearerToken")
                    })
                    .post_with(upload, |op| {
                        resource_op!(op, StorageDocs::Upload)
                            .security_requirement("BearerToken")
                            .response::<201, Json<StorageResponse>>()
                    }),
                )
                .api_route(
                    "/{ref_key}",
                    routing::get_with(show, |op| {
                        resource_op!(op, StorageDocs::Show).security_requirement("BearerToken")
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, StorageDocs::Remove).security_requirement("BearerToken")
                    }),
                ),
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

async fn list(
    context: ProjectContext,
    Query(params): Query<ListStorage>,
) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(ref_key): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    let key = if ref_key.starts_with("ref:") {
        ResourceKey::Ref(ref_key.parse().map_err(|_| StorageError::InvalidRef)?)
    } else {
        let storage_ref = StorageRef(ref_key);
        ResourceKey::Key(storage_ref.decode().map_err(|_| StorageError::InvalidRef)?)
    };
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
