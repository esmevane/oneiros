use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::{StatusCode, header},
    response::IntoResponse,
};

use crate::*;

pub(crate) struct StorageRouter;

impl StorageRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/storage",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, StorageDocs::List)
                            .security_requirement("BearerToken")
                            .response::<200, Json<StorageEntriesResponse>>()
                    })
                    .post_with(upload, |op| {
                        resource_op!(op, StorageDocs::Upload)
                            .security_requirement("BearerToken")
                            .response::<201, Json<StorageSetResponse>>()
                    }),
                )
                .api_route(
                    "/{ref_key}",
                    routing::get_with(show, |op| {
                        resource_op!(op, StorageDocs::Show)
                            .security_requirement("BearerToken")
                            .input::<RefKeyPathParam<StorageKey>>()
                            .response::<200, Json<StorageDetailsResponse>>()
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, StorageDocs::Remove)
                            .security_requirement("BearerToken")
                            .input::<RefKeyPathParam<StorageKey>>()
                            .response::<200, Json<StorageRemovedResponse>>()
                    }),
                )
                // Raw blob bytes — application/octet-stream, not JSON, so
                // it lives outside aide's typed routing.
                .route("/{ref_key}/content", axum::routing::get(content)),
        )
    }
}

async fn upload(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Json(body): Json<UploadStorage>,
) -> Result<(StatusCode, Json<StorageResponse>), StorageError> {
    let response = StorageService::upload(&scope, &mailbox, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    scope: Scope<AtBookmark>,
    Query(params): Query<ListStorage>,
) -> Result<Json<StorageResponse>, StorageError> {
    Ok(Json(StorageService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtBookmark>,
    Path(ref_key): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    let key = if ref_key.starts_with(REF_PREFIX) {
        ResourceKey::Ref(ref_key.parse().map_err(|_| StorageError::InvalidRef)?)
    } else {
        let storage_ref = StorageRef(ref_key);
        ResourceKey::Key(storage_ref.decode().map_err(|_| StorageError::InvalidRef)?)
    };
    Ok(Json(
        StorageService::show(&scope, &GetStorage::builder_v1().key(key).build().into()).await?,
    ))
}

async fn remove(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(ref_key): Path<String>,
) -> Result<Json<StorageResponse>, StorageError> {
    let storage_ref = StorageRef(ref_key);
    let key = storage_ref.decode().map_err(|_| StorageError::InvalidRef)?;
    Ok(Json(
        StorageService::remove(
            &scope,
            &mailbox,
            &RemoveStorage::builder_v1().key(key).build().into(),
        )
        .await?,
    ))
}

async fn content(
    scope: Scope<AtBookmark>,
    Path(ref_key): Path<String>,
) -> Result<impl IntoResponse, StorageError> {
    let storage_ref = StorageRef(ref_key);
    let key = storage_ref.decode().map_err(|_| StorageError::InvalidRef)?;
    let bytes = StorageService::get_content(&scope, &key).await?;
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/octet-stream")],
        bytes,
    ))
}
