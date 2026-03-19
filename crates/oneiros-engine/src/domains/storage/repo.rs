use rusqlite::{Connection, params};

use crate::*;

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

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Storage(storage_event) = &event.data {
            match storage_event {
                StorageEvents::BlobStored(entry) => self.create_record(entry)?,
                StorageEvents::BlobRemoved(removed) => self.remove(&removed.id)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM storage_entries", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
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

    pub fn get(&self, id: &str) -> Result<Option<StorageEntry>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, content_type, size, created_at FROM storage_entries WHERE id = ?1",
        )?;

        let raw = stmt.query_row(params![id], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let content_type: String = row.get(2)?;
            let size: i64 = row.get(3)?;
            let created_at: String = row.get(4)?;
            Ok((id, name, content_type, size, created_at))
        });

        match raw {
            Ok((id, name, content_type, size, created_at)) => Ok(Some(StorageEntry {
                id: id.parse()?,
                name: StorageName::new(name),
                content_type: Label::new(content_type),
                size: size as u64,
                created_at: Timestamp::parse_str(&created_at)?,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_by_name(&self, name: &str) -> Result<Option<StorageEntry>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, content_type, size, created_at FROM storage_entries WHERE name = ?1",
        )?;

        let raw = stmt.query_row(params![name], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let content_type: String = row.get(2)?;
            let size: i64 = row.get(3)?;
            let created_at: String = row.get(4)?;
            Ok((id, name, content_type, size, created_at))
        });

        match raw {
            Ok((id, name, content_type, size, created_at)) => Ok(Some(StorageEntry {
                id: id.parse()?,
                name: StorageName::new(name),
                content_type: Label::new(content_type),
                size: size as u64,
                created_at: Timestamp::parse_str(&created_at)?,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<StorageEntry>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, content_type, size, created_at FROM storage_entries ORDER BY name",
        )?;

        let raw: Vec<(String, String, String, i64, String)> = stmt
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut entries = vec![];

        for (id, name, content_type, size, created_at) in raw {
            entries.push(StorageEntry {
                id: id.parse()?,
                name: StorageName::new(name),
                content_type: Label::new(content_type),
                size: size as u64,
                created_at: Timestamp::parse_str(&created_at)?,
            });
        }

        Ok(entries)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, entry: &StorageEntry) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO storage_entries (id, name, content_type, size, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                entry.id.to_string(),
                entry.name.to_string(),
                entry.content_type.to_string(),
                entry.size as i64,
                entry.created_at.as_string()
            ],
        )?;
        Ok(())
    }

    fn remove(&self, id: &str) -> Result<(), EventError> {
        self.conn
            .execute("DELETE FROM storage_entries WHERE id = ?1", params![id])?;
        Ok(())
    }
}
