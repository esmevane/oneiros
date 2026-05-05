use crate::*;

pub struct StorageService;

impl StorageService {
    /// Upload content — hash it, store the blob, dispatch the metadata
    /// event, return the eventually-consistent entry.
    ///
    /// The blob table is the durable store, not a projection — the put
    /// happens here directly, outside the bus. The metadata event is
    /// the projection-driving signal that the bookmark actor materializes.
    pub async fn upload(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &UploadStorage,
    ) -> Result<StorageResponse, StorageError> {
        let UploadStorage::V1(upload) = request;
        let blob = BlobContent::create(&upload.data)?;

        // Put the blob directly — content-addressed storage, not a
        // projection. Direct write to the bookmark DB.
        let bookmark_db = BookmarkDb::open(scope).await?;
        StorageStore::new(&bookmark_db).put_blob(&blob)?;
        drop(bookmark_db);

        let entry = StorageEntry {
            key: upload.key.clone(),
            description: upload.description.clone(),
            hash: blob.hash.clone(),
        };
        let key = entry.key.clone();

        let new_event = NewEvent::builder()
            .data(Events::Storage(StorageEvents::StorageSet(
                StorageSet::builder_v1().entry(entry).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let stored = StorageRepo::new(scope)
            .fetch_storage(&key)
            .await?
            .ok_or(StorageError::KeyNotFound(key))?;

        Ok(StorageResponse::StorageSet(
            StorageSetResponse::builder_v1()
                .entry(stored)
                .build()
                .into(),
        ))
    }

    /// Show storage metadata by key.
    pub async fn show(
        scope: &Scope<AtBookmark>,
        request: &GetStorage,
    ) -> Result<StorageResponse, StorageError> {
        let GetStorage::V1(lookup) = request;
        let key = lookup.key.resolve()?;
        let entry = StorageRepo::new(scope)
            .fetch_storage(&key)
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
        scope: &Scope<AtBookmark>,
        request: &ListStorage,
    ) -> Result<StorageResponse, StorageError> {
        let ListStorage::V1(listing) = request;
        let listed = StorageRepo::new(scope)
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
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RemoveStorage,
    ) -> Result<StorageResponse, StorageError> {
        let RemoveStorage::V1(removal) = request;
        StorageRepo::new(scope)
            .fetch_storage(&removal.key)
            .await?
            .ok_or_else(|| StorageError::KeyNotFound(removal.key.clone()))?;

        let new_event = NewEvent::builder()
            .data(Events::Storage(StorageEvents::StorageRemoved(
                StorageRemoved::builder_v1()
                    .key(removal.key.clone())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        scope
            .config()
            .fetch
            .until_absent(|| async { StorageRepo::new(scope).fetch_storage(&removal.key).await })
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
        scope: &Scope<AtBookmark>,
        key: &StorageKey,
    ) -> Result<Vec<u8>, StorageError> {
        let repo = StorageRepo::new(scope);

        let entry = repo
            .fetch_storage(key)
            .await?
            .ok_or_else(|| StorageError::KeyNotFound(key.clone()))?;

        let blob = repo
            .fetch_blob(&entry.hash)
            .await?
            .ok_or_else(|| StorageError::BlobMissing(entry.hash.clone()))?;

        Ok(blob.data.decompressed()?)
    }
}
