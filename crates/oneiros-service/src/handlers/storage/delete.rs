use axum::extract::Path;
use axum::http::StatusCode;
use oneiros_model::StorageRef;
use oneiros_protocol::{Events, StorageEvents};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<StatusCode, Error> {
    let key = storage_ref
        .decode()
        .map_err(|e| Error::BadRequest(BadRequests::StorageRef(e)))?;

    let event = Events::Storage(StorageEvents::StorageRemoved { key });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok(StatusCode::OK)
}
