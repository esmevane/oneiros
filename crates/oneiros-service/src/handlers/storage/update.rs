use axum::{
    Json,
    body::Bytes,
    extract::Path,
    http::{HeaderMap, StatusCode},
};
use flate2::{Compression, write::ZlibEncoder};
use oneiros_model::*;
use sha2::{Digest, Sha256};
use std::io::Write;

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

    // Compute SHA-256 hash of raw content
    let mut hasher = Sha256::new();
    hasher.update(&body);
    let hash_bytes = hasher.finalize();
    let hash_hex = data_encoding::HEXLOWER.encode(&hash_bytes);

    // Compress with zlib
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&body)?;
    let compressed = encoder.finish()?;

    // Write blob (INSERT OR IGNORE — idempotent by hash)
    ticket.db.put_blob(&hash_hex, &compressed, body.len())?;

    // Log event (projection will update storage table)
    let entry = StorageEntry::init(key, description, ContentHash::new(&hash_hex));

    let event = Events::Storage(StorageEvents::StorageSet(entry.clone()));
    ticket.db.log_event(&event, projections::BRAIN)?;

    Ok((StatusCode::OK, Json(entry)))
}
