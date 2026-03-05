use axum::Json;
use axum::extract::Path;
use oneiros_model::{StorageRef, StorageResponses};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<Json<StorageResponses>, Error> {
    let key = storage_ref
        .decode()
        .map_err(oneiros_service::BadRequests::StorageRef)?;

    let response = ticket.service().get_storage(&key)?;

    Ok(Json(response))
}
