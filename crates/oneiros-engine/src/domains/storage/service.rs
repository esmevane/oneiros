use crate::*;

pub struct StorageService;

impl StorageService {
    /// Upload content — hash it, store the blob, record the metadata.
    pub async fn upload(
        context: &ProjectContext,
        key: StorageKey,
        description: Description,
        data: Vec<u8>,
    ) -> Result<StorageResponse, StorageError> {
        let blob = BlobContent::create(&data)?;

        // Put the blob directly (not via event — the blob table is the durable store).
        StorageRepo::new(&context.db()?).put_blob(&blob)?;

        let entry = StorageEntry {
            key,
            description,
            hash: blob.hash,
        };

        // Emit StorageSet — this drives the storage metadata projection.
        context
            .emit(StorageEvents::StorageSet(entry.clone()))
            .await?;

        Ok(StorageResponse::StorageSet(entry))
    }

    /// Show storage metadata by key.
    pub fn show(
        context: &ProjectContext,
        key: &StorageKey,
    ) -> Result<StorageResponse, StorageError> {
        let entry = StorageRepo::new(&context.db()?)
            .get_storage(key)?
            .ok_or_else(|| StorageError::KeyNotFound(key.clone()))?;

        Ok(StorageResponse::StorageDetails(entry))
    }

    /// List all storage entries.
    pub fn list(context: &ProjectContext) -> Result<StorageResponse, StorageError> {
        let entries = StorageRepo::new(&context.db()?).list_storage()?;

        if entries.is_empty() {
            Ok(StorageResponse::NoEntries)
        } else {
            Ok(StorageResponse::Entries(entries))
        }
    }

    /// Remove storage metadata by key. The blob is NOT deleted (dedup preservation).
    pub async fn remove(
        context: &ProjectContext,
        key: &StorageKey,
    ) -> Result<StorageResponse, StorageError> {
        // Confirm the key exists before emitting.
        StorageRepo::new(&context.db()?)
            .get_storage(key)?
            .ok_or_else(|| StorageError::KeyNotFound(key.clone()))?;

        context
            .emit(StorageEvents::StorageRemoved(SelectStorageByKey {
                key: key.clone(),
            }))
            .await?;

        Ok(StorageResponse::StorageRemoved(key.clone()))
    }

    /// Retrieve the raw binary content for a storage key.
    pub fn get_content(
        context: &ProjectContext,
        key: &StorageKey,
    ) -> Result<Vec<u8>, StorageError> {
        let db = context.db()?;
        let repo = StorageRepo::new(&db);

        let entry = repo
            .get_storage(key)?
            .ok_or_else(|| StorageError::KeyNotFound(key.clone()))?;

        let blob = repo
            .get_blob(&entry.hash)?
            .ok_or_else(|| StorageError::BlobMissing(entry.hash.clone()))?;

        Ok(blob.data.decompressed()?)
    }
}
