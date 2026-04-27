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

        let entry = StorageEntry::Current(
            StorageEntry::build_v1()
                .key(key.clone())
                .description(description.clone())
                .hash(blob.hash().clone())
                .build(),
        );

        // Emit StorageSet — this drives the storage metadata projection.
        context
            .emit(StorageEvents::StorageSet(entry.clone()))
            .await?;

        let ref_token = RefToken::new(Ref::storage(entry.key().clone()));
        Ok(StorageResponse::StorageSet(
            Response::new(entry).with_ref_token(ref_token),
        ))
    }

    /// Show storage metadata by key.
    pub async fn show(
        context: &ProjectContext,
        selector: &GetStorage,
    ) -> Result<StorageResponse, StorageError> {
        let key = selector.key.resolve()?;
        let entry = StorageRepo::new(context)
            .get_storage(&key)
            .await?
            .ok_or(StorageError::KeyNotFound(key))?;
        let ref_token = RefToken::new(Ref::storage(entry.key().clone()));
        Ok(StorageResponse::StorageDetails(
            Response::new(entry).with_ref_token(ref_token),
        ))
    }

    /// List all storage entries.
    pub async fn list(
        context: &ProjectContext,
        ListStorage { filters }: &ListStorage,
    ) -> Result<StorageResponse, StorageError> {
        let listed = StorageRepo::new(context).list_storage(filters).await?;

        if listed.total == 0 {
            Ok(StorageResponse::NoEntries)
        } else {
            Ok(StorageResponse::Entries(listed.map(|e| {
                let ref_token = RefToken::new(Ref::storage(e.key().clone()));
                Response::new(e).with_ref_token(ref_token)
            })))
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
            .emit(StorageEvents::StorageRemoved(SelectStorageByKey::Current(
                SelectStorageByKeyV1 {
                    key: selector.key.clone(),
                },
            )))
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
            .get_blob(entry.hash())
            .await?
            .ok_or_else(|| StorageError::BlobMissing(entry.hash().clone()))?;

        Ok(blob.data().decompressed()?)
    }
}
