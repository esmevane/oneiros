use crate::*;

pub struct StorageService;

impl StorageService {
    /// Upload bytes to the filesystem and record metadata in the event log.
    ///
    /// Binary data is written to `data_dir/blobs/{id}`. Metadata is recorded
    /// via a `blob-stored` event which runs through projections.
    pub fn upload(
        context: &ProjectContext,
        name: StorageName,
        content_type: Label,
        data: Vec<u8>,
    ) -> Result<StorageResponse, StorageError> {
        let data_dir = context.data_dir().ok_or(StorageError::NoDataDir)?;

        let id = StorageId::new();
        let blobs_dir = data_dir.join("blobs");
        std::fs::create_dir_all(&blobs_dir)?;

        let blob_path = blobs_dir.join(id.to_string());
        std::fs::write(&blob_path, &data)?;

        let entry = StorageEntry {
            id,
            name: name.clone(),
            content_type,
            size: data.len() as u64,
            created_at: Timestamp::now(),
        };

        context.emit(StorageEvents::BlobStored(entry.clone()));
        Ok(StorageResponse::StorageSet(name))
    }

    pub fn show(
        context: &ProjectContext,
        key: &StorageName,
    ) -> Result<StorageResponse, StorageError> {
        let entry = context
            .with_db(|conn| StorageRepo::new(conn).get_by_name(key))?
            .ok_or_else(|| StorageError::NameNotFound(key.clone()))?;
        Ok(StorageResponse::StorageDetails(entry))
    }

    pub fn get(context: &ProjectContext, id: &StorageId) -> Result<StorageResponse, StorageError> {
        let entry = context
            .with_db(|conn| StorageRepo::new(conn).get(id))?
            .ok_or_else(|| StorageError::NotFound(id.clone()))?;
        Ok(StorageResponse::StorageDetails(entry))
    }

    /// Retrieve the binary content of a stored blob.
    pub fn get_content(
        context: &ProjectContext,
        id: &StorageId,
    ) -> Result<StorageResponse, StorageError> {
        let data_dir = context.data_dir().ok_or(StorageError::NoDataDir)?;

        let entry = context
            .with_db(|conn| StorageRepo::new(conn).get(id))?
            .ok_or_else(|| StorageError::NotFound(id.clone()))?;

        let blob_path = data_dir.join("blobs").join(id.to_string());
        let data = std::fs::read(&blob_path)?;

        Ok(StorageResponse::Content(StorageContent { entry, data }))
    }

    pub fn list(context: &ProjectContext) -> Result<StorageResponse, StorageError> {
        let entries = context
            .with_db(|conn| StorageRepo::new(conn).list())
            .map_err(StorageError::Database)?;
        if entries.is_empty() {
            Ok(StorageResponse::NoEntries)
        } else {
            Ok(StorageResponse::Entries(entries))
        }
    }

    /// Remove the metadata record and delete the blob file from the filesystem.
    pub fn remove(
        context: &ProjectContext,
        id: &StorageId,
    ) -> Result<StorageResponse, StorageError> {
        let data_dir = context.data_dir().ok_or(StorageError::NoDataDir)?;

        // Confirm existence before emitting.
        let entry = context
            .with_db(|conn| StorageRepo::new(conn).get(id))?
            .ok_or_else(|| StorageError::NotFound(id.clone()))?;

        // Remove the file — best effort; log the event even if the file is missing.
        let blob_path = data_dir.join("blobs").join(id.to_string());
        if blob_path.exists() {
            std::fs::remove_file(&blob_path)?;
        }

        context.emit(StorageEvents::BlobRemoved(BlobRemoved { id: id.clone() }));
        Ok(StorageResponse::StorageRemoved(entry.name))
    }
}
