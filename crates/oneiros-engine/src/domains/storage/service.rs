use crate::*;

pub struct StorageService;

impl StorageService {
    /// Upload bytes to the filesystem and record metadata in the event log.
    ///
    /// Binary data is written to `data_dir/blobs/{id}`. Metadata is recorded
    /// via a `blob-stored` event which runs through projections.
    pub fn upload(
        ctx: &ProjectContext,
        name: String,
        content_type: String,
        data: Vec<u8>,
    ) -> Result<StorageResponse, StorageError> {
        let data_dir = ctx.data_dir().ok_or(StorageError::NoDataDir)?;

        let id = StorageId::new();
        let blobs_dir = data_dir.join("blobs");
        std::fs::create_dir_all(&blobs_dir)?;

        let blob_path = blobs_dir.join(id.to_string());
        std::fs::write(&blob_path, &data)?;

        let storage_name = StorageName::new(name.clone());
        let entry = StorageEntry {
            id,
            name: storage_name.clone(),
            content_type: Label::new(content_type),
            size: data.len() as u64,
            created_at: Timestamp::now(),
        };

        ctx.emit(StorageEvents::BlobStored(entry.clone()));
        Ok(StorageResponse::StorageSet(storage_name))
    }

    pub fn show(ctx: &ProjectContext, key: &str) -> Result<StorageResponse, StorageError> {
        let entry = ctx
            .with_db(|conn| StorageRepo::new(conn).get_by_name(key))
            .map_err(StorageError::Database)?
            .ok_or_else(|| StorageError::NotFound(key.to_string()))?;
        Ok(StorageResponse::StorageDetails(entry))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<StorageResponse, StorageError> {
        let entry = ctx
            .with_db(|conn| StorageRepo::new(conn).get(id))
            .map_err(StorageError::Database)?
            .ok_or_else(|| StorageError::NotFound(id.to_string()))?;
        Ok(StorageResponse::StorageDetails(entry))
    }

    /// Retrieve the binary content of a stored blob.
    pub fn get_content(ctx: &ProjectContext, id: &str) -> Result<StorageResponse, StorageError> {
        let data_dir = ctx.data_dir().ok_or(StorageError::NoDataDir)?;

        let entry = ctx
            .with_db(|conn| StorageRepo::new(conn).get(id))
            .map_err(StorageError::Database)?
            .ok_or_else(|| StorageError::NotFound(id.to_string()))?;

        let blob_path = data_dir.join("blobs").join(id);
        let data = std::fs::read(&blob_path)?;

        Ok(StorageResponse::Content(StorageContent { entry, data }))
    }

    pub fn list(ctx: &ProjectContext) -> Result<StorageResponse, StorageError> {
        let entries = ctx
            .with_db(|conn| StorageRepo::new(conn).list())
            .map_err(StorageError::Database)?;
        if entries.is_empty() {
            Ok(StorageResponse::NoEntries)
        } else {
            Ok(StorageResponse::Entries(entries))
        }
    }

    /// Remove the metadata record and delete the blob file from the filesystem.
    pub fn remove(ctx: &ProjectContext, id: &str) -> Result<StorageResponse, StorageError> {
        let data_dir = ctx.data_dir().ok_or(StorageError::NoDataDir)?;

        // Confirm existence before emitting.
        let entry = ctx
            .with_db(|conn| StorageRepo::new(conn).get(id))
            .map_err(StorageError::Database)?
            .ok_or_else(|| StorageError::NotFound(id.to_string()))?;

        // Remove the file — best effort; log the event even if the file is missing.
        let blob_path = data_dir.join("blobs").join(id);
        if blob_path.exists() {
            std::fs::remove_file(&blob_path)?;
        }

        ctx.emit(StorageEvents::BlobRemoved(BlobRemoved {
            id: id.to_string(),
        }));
        Ok(StorageResponse::StorageRemoved(entry.name))
    }
}
