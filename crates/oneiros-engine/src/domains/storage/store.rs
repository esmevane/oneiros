use rusqlite::params;

use crate::*;

/// Storage store — projection lifecycle, blob writes, and metadata write operations.
///
/// Two-tier design (inspired by Fossil):
/// - `blob` table: content-addressed by SHA256 hash, stores compressed binary
/// - `storage` table: maps human-readable keys to blob hashes
pub struct StorageStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> StorageStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Storage(storage_event)) = &event.data {
            match storage_event {
                StorageEvents::StorageSet(entry) => self.set_storage(entry)?,
                StorageEvents::StorageRemoved(removed) => self.remove_storage(&removed.key)?,
            }
        }
        Ok(())
    }

    pub fn reset_blobs(&self) -> Result<(), EventError> {
        self.conn.execute_batch("DELETE FROM blob")?;
        Ok(())
    }

    pub fn reset_storage(&self) -> Result<(), EventError> {
        self.conn.execute_batch("DELETE FROM storage")?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS blob (
                hash TEXT PRIMARY KEY NOT NULL,
                data BLOB NOT NULL,
                size INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS storage (
                key TEXT PRIMARY KEY NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                hash TEXT NOT NULL REFERENCES blob(hash)
            )",
        )?;
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(hash = %content.hash, size = content.size.as_i64()), err(Display))]
    pub fn put_blob(&self, content: &BlobContent) -> Result<(), EventError> {
        let bytes = content.data.decode()?;
        self.conn.execute(
            "INSERT OR IGNORE INTO blob (hash, data, size) VALUES (?1, ?2, ?3)",
            params![content.hash.as_str(), &bytes, content.size.as_i64()],
        )?;
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(hash = %hash), err(Display))]
    pub fn get_blob(&self, hash: &ContentHash) -> Result<Option<BlobContent>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT hash, data, size FROM blob WHERE hash = ?1")?;

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

    pub fn set_storage(&self, entry: &StorageEntry) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT INTO storage (key, description, hash) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET description = excluded.description, hash = excluded.hash",
            params![
                entry.key.as_str(),
                entry.description.as_str(),
                entry.hash.as_str(),
            ],
        )?;
        Ok(())
    }

    pub fn remove_storage(&self, key: &StorageKey) -> Result<(), EventError> {
        self.conn
            .execute("DELETE FROM storage WHERE key = ?1", params![key.as_str()])?;
        Ok(())
    }
}
