use crate::*;

pub struct StorageService;

impl StorageService {
    /// Upload content — hash it, store the blob, record the metadata.
    pub async fn upload(
        context: &ProjectContext,
        request: &UploadStorage,
    ) -> Result<StorageResponse, StorageError> {
        let UploadStorage::V1(upload) = request;
        let blob = BlobContent::create(&upload.data)?;

        // Put the blob directly (not via event — the blob table is the durable store).
        StorageStore::new(&context.db()?).put_blob(&blob)?;

        let entry = StorageEntry {
            key: upload.key.clone(),
            description: upload.description.clone(),
            hash: blob.hash.clone(),
        };

        // Emit StorageSet — this drives the storage metadata projection.
        context
            .emit(StorageEvents::StorageSet(
                StorageSet::builder_v1().entry(entry.clone()).build().into(),
            ))
            .await?;

        Ok(StorageResponse::StorageSet(
            StorageSetResponse::builder_v1().entry(entry).build().into(),
        ))
    }

    /// Show storage metadata by key.
    pub async fn show(
        context: &ProjectContext,
        request: &GetStorage,
    ) -> Result<StorageResponse, StorageError> {
        let GetStorage::V1(lookup) = request;
        let key = lookup.key.resolve()?;
        let entry = StorageRepo::new(context)
            .get_storage(&key)
            .await?
            .ok_or(StorageError::KeyNotFound(key))?;
        Ok(StorageResponse::StorageDetails(
            StorageDetailsResponse::builder_v1()
                .entry(entry)
                .build()
                .into(),
        ))
    }

    /// List all storage entries.
    pub async fn list(
        context: &ProjectContext,
        request: &ListStorage,
    ) -> Result<StorageResponse, StorageError> {
        let ListStorage::V1(listing) = request;
        let listed = StorageRepo::new(context)
            .list_storage(&listing.filters)
            .await?;

        if listed.total == 0 {
            Ok(StorageResponse::NoEntries)
        } else {
            Ok(StorageResponse::Entries(
                StorageEntriesResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            ))
        }
    }

    /// Remove storage metadata by key. The blob is NOT deleted (dedup preservation).
    pub async fn remove(
        context: &ProjectContext,
        request: &RemoveStorage,
    ) -> Result<StorageResponse, StorageError> {
        let RemoveStorage::V1(removal) = request;
        // Confirm the key exists before emitting.
        StorageRepo::new(context)
            .get_storage(&removal.key)
            .await?
            .ok_or_else(|| StorageError::KeyNotFound(removal.key.clone()))?;

        context
            .emit(StorageEvents::StorageRemoved(
                StorageRemoved::builder_v1()
                    .key(removal.key.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(StorageResponse::StorageRemoved(
            StorageRemovedResponse::builder_v1()
                .key(removal.key.clone())
                .build()
                .into(),
        ))
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
