use axum::{
    Json, Router,
    body::Bytes,
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response as AxumResponse},
    routing,
};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::get(index))
        .route("/{key}", routing::put(update))
        .route("/{key}", routing::get(show))
        .route("/{key}", routing::delete(delete))
        .route("/{key}/content", routing::get(content))
}

async fn index(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(StorageRequests::ListStorage(
        ListStorageRequest,
    ))?))
}

async fn update(
    ticket: OneirosContext,
    Path(storage_ref): Path<StorageRef>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(StatusCode, Json<Response>), Error> {
    let key = storage_ref
        .decode()
        .map_err(oneiros_service::BadRequests::StorageRef)?;

    let description = headers
        .get("x-storage-description")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let response = ticket.set_storage(SetStorageRequest {
        key,
        description,
        data: body.to_vec(),
    })?;

    Ok((StatusCode::OK, Json(response)))
}

async fn show(
    ticket: OneirosContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<Json<Response>, Error> {
    let key = storage_ref
        .decode()
        .map_err(oneiros_service::BadRequests::StorageRef)?;

    Ok(Json(ticket.dispatch(StorageRequests::GetStorage(
        GetStorageRequest { key },
    ))?))
}

async fn delete(
    ticket: OneirosContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<Json<Response>, Error> {
    let key = storage_ref
        .decode()
        .map_err(oneiros_service::BadRequests::StorageRef)?;

    Ok(Json(ticket.dispatch(StorageRequests::RemoveStorage(
        RemoveStorageRequest { key },
    ))?))
}

async fn content(
    ticket: OneirosContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<AxumResponse, Error> {
    let key = storage_ref
        .decode()
        .map_err(oneiros_service::BadRequests::StorageRef)?;

    let data = ticket.get_storage_content(&key)?;

    Ok((
        StatusCode::OK,
        [("content-type", "application/octet-stream")],
        data,
    )
        .into_response())
}
