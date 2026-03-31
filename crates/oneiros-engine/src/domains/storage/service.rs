use crate::*;

pub struct StorageService;

impl StorageService {
    /// Upload content — hash it, store the blob, record the metadata.
    pub async fn upload(
        context: &ProjectContext,
        UploadStorage {
            key,
            description,
            data,
        }: &UploadStorage,
    ) -> Result<StorageResponse, StorageError> {
        let blob = BlobContent::create(data)?;

        // Put the blob directly (not via event — the blob table is the durable store).
        StorageStore::new(&context.db()?).put_blob(&blob)?;

        let entry = StorageEntry {
            key: key.clone(),
            description: description.clone(),
            hash: blob.hash,
        };

        // Emit StorageSet — this drives the storage metadata projection.
        context
            .emit(StorageEvents::StorageSet(entry.clone()))
            .await?;

        Ok(StorageResponse::StorageSet(entry))
    }

    /// Show storage metadata by key.
    pub async fn show(
        context: &ProjectContext,
        selector: &GetStorage,
    ) -> Result<StorageResponse, StorageError> {
        let entry = StorageRepo::new(context)
            .get_storage(&selector.key)
            .await?
            .ok_or_else(|| StorageError::KeyNotFound(selector.key.clone()))?;

        Ok(StorageResponse::StorageDetails(entry))
    }

    /// List all storage entries.
    pub async fn list(context: &ProjectContext) -> Result<StorageResponse, StorageError> {
        let entries = StorageRepo::new(context).list_storage().await?;

        if entries.is_empty() {
            Ok(StorageResponse::NoEntries)
        } else {
            Ok(StorageResponse::Entries(entries))
        }
    }

    /// Remove storage metadata by key. The blob is NOT deleted (dedup preservation).
    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveStorage,
    ) -> Result<StorageResponse, StorageError> {
        // Confirm the key exists before emitting.
        StorageRepo::new(context)
            .get_storage(&selector.key)
            .await?
            .ok_or_else(|| StorageError::KeyNotFound(selector.key.clone()))?;

        context
            .emit(StorageEvents::StorageRemoved(SelectStorageByKey {
                key: selector.key.clone(),
            }))
            .await?;

        Ok(StorageResponse::StorageRemoved(selector.key.clone()))
    }

    /// Retrieve the raw binary content for a storage key.
    pub async fn get_content(
        context: &ProjectContext,
        key: &StorageKey,
    ) -> Result<Vec<u8>, StorageError> {
        let repo = StorageRepo::new(context);

        let entry = repo
            .get_storage(key)
            .await?
            .ok_or_else(|| StorageError::KeyNotFound(key.clone()))?;

        let blob = repo
            .get_blob(&entry.hash)
            .await?
            .ok_or_else(|| StorageError::BlobMissing(entry.hash.clone()))?;

        Ok(blob.data.decompressed()?)
    }
}
