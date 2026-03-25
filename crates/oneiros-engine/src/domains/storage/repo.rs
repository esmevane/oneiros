use rusqlite::params;

use crate::*;

/// Storage repo — async read queries over the storage and blob projection tables.
pub struct StorageRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> StorageRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    // ── Read queries ────────────────────────────────────────────

    pub async fn get_storage(&self, key: &StorageKey) -> Result<Option<StorageEntry>, EventError> {
        let db = self.context.db()?;
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

    pub async fn list_storage(&self) -> Result<Vec<StorageEntry>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare("SELECT key, description, hash FROM storage ORDER BY key")?;

        let entries = stmt
            .query_map([], |row| {
                let key: String = row.get(0)?;
                let description: String = row.get(1)?;
                let hash: String = row.get(2)?;
                Ok((key, description, hash))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries
            .into_iter()
            .map(|(key, description, hash)| StorageEntry {
                key: StorageKey::new(key),
                description: Description::new(description),
                hash: ContentHash::new(hash),
            })
            .collect())
    }

    pub async fn get_blob(&self, hash: &ContentHash) -> Result<Option<BlobContent>, EventError> {
        let db = self.context.db()?;
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
