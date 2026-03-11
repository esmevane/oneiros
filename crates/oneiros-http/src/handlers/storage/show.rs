use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<Json<Response>, Error> {
    let key = storage_ref
        .decode()
        .map_err(oneiros_service::BadRequests::StorageRef)?;

    Ok(Json(ticket.dispatch(StorageRequests::GetStorage(
        GetStorageRequest { key },
    ))?))
}
