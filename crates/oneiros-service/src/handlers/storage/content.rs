use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use flate2::read::ZlibDecoder;
use oneiros_model::{Key, StorageRef};
use std::io::Read;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(storage_ref): Path<StorageRef>,
) -> Result<Response, Error> {
    let key = storage_ref
        .decode()
        .map_err(|e| Error::BadRequest(BadRequests::StorageRef(e)))?;

    let entry = ticket
        .db
        .get_storage(&key)?
        .ok_or(NotFound::Storage(Key::Id(key)))?;

    let (compressed, _original_size) = ticket
        .db
        .get_blob(&entry.hash)?
        .ok_or(DataIntegrity::BlobMissing(entry.hash))?;

    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    Ok((
        StatusCode::OK,
        [("content-type", "application/octet-stream")],
        decompressed,
    )
        .into_response())
}
