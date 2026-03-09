use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<StorageResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_storage(StorageRequests::ListStorage(ListStorageRequest))?;

    Ok(Json(response))
}
