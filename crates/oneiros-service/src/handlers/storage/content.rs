use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use oneiros_model::StorageRef;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<Response, Error> {
    let key = storage_ref
        .decode()
        .map_err(|e| Error::BadRequest(BadRequests::StorageRef(e)))?;

    let data = ticket.service().get_storage_content(&key)?;

    Ok((
        StatusCode::OK,
        [("content-type", "application/octet-stream")],
        data,
    )
        .into_response())
}
