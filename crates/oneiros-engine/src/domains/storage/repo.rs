use rusqlite::params;

use crate::*;

/// Storage repo — async read queries over the storage and blob projection tables.
pub struct StorageRepo<'a> {
    scope: &'a Scope<AtBookmark>,
}

impl<'a> StorageRepo<'a> {
    pub fn new(scope: &'a Scope<AtBookmark>) -> Self {
        Self { scope }
    }

    /// Eventually-consistent variant of [`get_storage`]. Polls until
    /// the entry appears or the configured patience window expires.
    ///
    /// [`get_storage`]: StorageRepo::get_storage
    pub async fn fetch_storage(
        &self,
        key: &StorageKey,
    ) -> Result<Option<StorageEntry>, EventError> {
        self.scope
            .config()
            .fetch
            .eventual(|| self.get_storage(key))
            .await
    }

    pub async fn get_storage(&self, key: &StorageKey) -> Result<Option<StorageEntry>, EventError> {
        let db = self.scope.bookmark_db().await?;
        let mut stmt = db.prepare("SELECT key, description, hash FROM storage WHERE key = ?1")?;

        let result = stmt.query_row(params![key.as_str()], |row| {
            let key: String = row.get(0)?;
            let description: String = row.get(1)?;
            let hash: String = row.get(2)?;
            Ok((key, description, hash))
        });

        match result {
            Ok((key, description, hash)) => Ok(Some(StorageEntry {
                key: StorageKey::new(key),
                description: Description::new(description),
                hash: ContentHash::new(hash),
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list_storage(
        &self,
        filters: &SearchFilters,
    ) -> Result<Listed<StorageEntry>, EventError> {
        let db = self.scope.bookmark_db().await?;

        let total: usize = db.query_row("SELECT COUNT(*) FROM storage", [], |row| row.get(0))?;

        let mut stmt = db.prepare(
            "SELECT key, description, hash FROM storage ORDER BY key LIMIT ?1 OFFSET ?2",
        )?;

        let entries = stmt
            .query_map(params![filters.limit, filters.offset], |row| {
                let key: String = row.get(0)?;
                let description: String = row.get(1)?;
                let hash: String = row.get(2)?;
                Ok((key, description, hash))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let items = entries
            .into_iter()
            .map(|(key, description, hash)| StorageEntry {
                key: StorageKey::new(key),
                description: Description::new(description),
                hash: ContentHash::new(hash),
            })
            .collect();

        Ok(Listed::new(items, total))
    }

    /// Eventually-consistent variant of [`get_blob`]. Polls until the
    /// blob appears or the configured patience window expires.
    ///
    /// [`get_blob`]: StorageRepo::get_blob
    pub async fn fetch_blob(&self, hash: &ContentHash) -> Result<Option<BlobContent>, EventError> {
        self.scope
            .config()
            .fetch
            .eventual(|| self.get_blob(hash))
            .await
    }

    pub async fn get_blob(&self, hash: &ContentHash) -> Result<Option<BlobContent>, EventError> {
        let db = self.scope.bookmark_db().await?;
        let mut stmt = db.prepare("SELECT hash, data, size FROM blob WHERE hash = ?1")?;

        let result = stmt.query_row(params![hash.as_str()], |row| {
            let hash: String = row.get(0)?;
            let data: Vec<u8> = row.get(1)?;
            let size: i64 = row.get(2)?;
            Ok((hash, data, size))
        });

        match result {
            Ok((hash, data, size)) => Ok(Some(BlobContent {
                hash: ContentHash::new(hash),
                size: Size::new(size as usize),
                data: Blob::encode(&data),
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
