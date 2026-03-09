use axum::extract::Path;
use axum::http::StatusCode;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<StatusCode, Error> {
    let key = storage_ref
        .decode()
        .map_err(oneiros_service::BadRequests::StorageRef)?;

    ticket
        .service()
        .dispatch_storage(StorageRequests::RemoveStorage(RemoveStorageRequest { key }))?;

    Ok(StatusCode::OK)
}
