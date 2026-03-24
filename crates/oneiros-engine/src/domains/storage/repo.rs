use rusqlite::{Connection, params};

use crate::event_log::EventLog;
use crate::*;

/// Storage read model — content-addressed blob storage with human-readable key mapping.
///
/// Two-tier design (inspired by Fossil):
/// - `blob` table: content-addressed by SHA256 hash, stores compressed binary
/// - `storage` table: maps human-readable keys to blob hashes
pub struct StorageRepo<'a> {
    conn: &'a Connection,
}

impl<'a> StorageRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle_blob_stored(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Storage(StorageEvents::BlobStored(content)) = &event.data {
            self.put_blob(content)?;
            // Delete the transient event — the blob is materialized, the event
            // served its purpose (carrying binary for export/import portability).
            EventLog::new(self.conn).delete(&event.id)?;
        }
        Ok(())
    }

    pub fn handle_storage_set(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Storage(StorageEvents::StorageSet(entry)) = &event.data {
            self.set_storage(entry)?;
        }
        Ok(())
    }

    pub fn handle_storage_removed(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Storage(StorageEvents::StorageRemoved(removed)) = &event.data {
            self.remove_storage(&removed.key)?;
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

    // ── Blob operations (content-addressed) ─────────────────────

    pub fn put_blob(&self, content: &BlobContent) -> Result<(), EventError> {
        let bytes = content.data.decode()?;
        self.conn.execute(
            "INSERT OR IGNORE INTO blob (hash, data, size) VALUES (?1, ?2, ?3)",
            params![content.hash.as_str(), &bytes, content.size.as_i64()],
        )?;
        Ok(())
    }

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

    // ── Storage metadata operations ─────────────────────────────

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

    pub fn get_storage(&self, key: &StorageKey) -> Result<Option<StorageEntry>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT key, description, hash FROM storage WHERE key = ?1")?;

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

    pub fn list_storage(&self) -> Result<Vec<StorageEntry>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT key, description, hash FROM storage ORDER BY key")?;

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

    pub fn remove_storage(&self, key: &StorageKey) -> Result<(), EventError> {
        self.conn
            .execute("DELETE FROM storage WHERE key = ?1", params![key.as_str()])?;
        Ok(())
    }
}
