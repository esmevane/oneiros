use axum::{
    Json,
    body::Bytes,
    extract::Path,
    http::{HeaderMap, StatusCode},
};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(StatusCode, Json<StorageEntry>), Error> {
    let key = storage_ref
        .decode()
        .map_err(|e| Error::BadRequest(BadRequests::StorageRef(e)))?;

    let description = headers
        .get("x-storage-description")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let entry = ticket.service().set_storage(key, description, &body)?;

    Ok((StatusCode::OK, Json(entry)))
}
