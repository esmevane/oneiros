use rusqlite::{Connection, params};

use crate::store::{StoreError, StoredEvent};

use super::model::StorageEntry;

/// Storage read model — queries, projection handling, and lifecycle.
///
/// Only metadata lives in the database. Binary data lives on the filesystem.
pub struct StorageRepo<'a> {
    conn: &'a Connection,
}

impl<'a> StorageRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        match event.event_type.as_str() {
            "blob-stored" => {
                let entry: StorageEntry = serde_json::from_value(event.data.clone())?;
                self.create_record(&entry)?;
            }
            "blob-removed" => {
                if let Some(id) = event.data.get("id").and_then(|v| v.as_str()) {
                    self.remove(id)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM storage_entries", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS storage_entries (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                content_type TEXT NOT NULL DEFAULT '',
                size INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &str) -> Result<Option<StorageEntry>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, content_type, size, created_at FROM storage_entries WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id], |row| {
            Ok(StorageEntry {
                id: row.get(0)?,
                name: row.get(1)?,
                content_type: row.get(2)?,
                size: row.get::<_, i64>(3)? as u64,
                created_at: row.get(4)?,
            })
        });

        match result {
            Ok(entry) => Ok(Some(entry)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<StorageEntry>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, content_type, size, created_at FROM storage_entries ORDER BY name",
        )?;

        let entries = stmt
            .query_map([], |row| {
                Ok(StorageEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    content_type: row.get(2)?,
                    size: row.get::<_, i64>(3)? as u64,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, entry: &StorageEntry) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO storage_entries (id, name, content_type, size, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                entry.id,
                entry.name,
                entry.content_type,
                entry.size as i64,
                entry.created_at
            ],
        )?;
        Ok(())
    }

    fn remove(&self, id: &str) -> Result<(), StoreError> {
        self.conn
            .execute("DELETE FROM storage_entries WHERE id = ?1", params![id])?;
        Ok(())
    }
}
