use std::path::Path;

use chrono::Utc;
use uuid::Uuid;

use crate::contexts::ProjectContext;

use super::errors::StorageError;
use super::model::StorageEntry;
use super::repo::StorageRepo;
use super::responses::StorageResponse;

pub struct StorageService;

impl StorageService {
    /// Upload bytes to the filesystem and record metadata in the event log.
    ///
    /// Binary data is written to `data_dir/blobs/{id}`. Metadata is recorded
    /// via a `blob-stored` event which runs through projections.
    pub fn upload(
        ctx: &ProjectContext,
        data_dir: &Path,
        name: String,
        content_type: String,
        data: Vec<u8>,
    ) -> Result<StorageResponse, StorageError> {
        let id = Uuid::now_v7().to_string();
        let blobs_dir = data_dir.join("blobs");
        std::fs::create_dir_all(&blobs_dir)?;

        let blob_path = blobs_dir.join(&id);
        std::fs::write(&blob_path, &data)?;

        let entry = StorageEntry {
            id,
            name,
            content_type,
            size: data.len() as u64,
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit("blob-stored", &entry);
        Ok(StorageResponse::Uploaded(entry))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<StorageResponse, StorageError> {
        let entry = ctx
            .with_db(|conn| StorageRepo::new(conn).get(id))
            .map_err(StorageError::Database)?
            .ok_or_else(|| StorageError::NotFound(id.to_string()))?;
        Ok(StorageResponse::Found(entry))
    }

    pub fn list(ctx: &ProjectContext) -> Result<StorageResponse, StorageError> {
        let entries = ctx
            .with_db(|conn| StorageRepo::new(conn).list())
            .map_err(StorageError::Database)?;
        Ok(StorageResponse::Listed(entries))
    }

    /// Remove the metadata record and delete the blob file from the filesystem.
    pub fn remove(
        ctx: &ProjectContext,
        data_dir: &Path,
        id: &str,
    ) -> Result<StorageResponse, StorageError> {
        // Confirm existence before emitting.
        let _ = ctx
            .with_db(|conn| StorageRepo::new(conn).get(id))
            .map_err(StorageError::Database)?
            .ok_or_else(|| StorageError::NotFound(id.to_string()))?;

        // Remove the file — best effort; log the event even if the file is missing.
        let blob_path = data_dir.join("blobs").join(id);
        if blob_path.exists() {
            std::fs::remove_file(&blob_path)?;
        }

        ctx.emit("blob-removed", &serde_json::json!({ "id": id }));
        Ok(StorageResponse::Removed)
    }
}
