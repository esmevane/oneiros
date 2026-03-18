use rusqlite::{Connection, params};

use crate::events::Events;
use crate::store::{StoreError, StoredEvent};

use super::events::*;
use super::model::Memory;

/// Memory read model — queries, projection handling, and lifecycle.
pub struct MemoryRepo<'a> {
    conn: &'a Connection,
}

impl<'a> MemoryRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if let Events::Memory(memory_event) = &event.data {
            match memory_event {
                MemoryEvents::MemoryAdded(memory) => self.insert(memory)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM memories", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS memories (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                level TEXT NOT NULL DEFAULT '',
                content TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &str) -> Result<Option<Memory>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, level, content, created_at
             FROM memories WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id], |row| {
            Ok(Memory {
                id: row.get(0)?,
                agent_id: row.get(1)?,
                level: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        });

        match result {
            Ok(m) => Ok(Some(m)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self, agent: Option<&str>) -> Result<Vec<Memory>, StoreError> {
        let mut stmt = match agent {
            Some(_) => self.conn.prepare(
                "SELECT id, agent_id, level, content, created_at
                 FROM memories WHERE agent_id = ?1 ORDER BY created_at",
            )?,
            None => self.conn.prepare(
                "SELECT id, agent_id, level, content, created_at
                 FROM memories ORDER BY created_at",
            )?,
        };

        let map_row = |row: &rusqlite::Row<'_>| {
            Ok(Memory {
                id: row.get(0)?,
                agent_id: row.get(1)?,
                level: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        };

        let memories = match agent {
            Some(a) => stmt.query_map(params![a], map_row),
            None => stmt.query_map([], map_row),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(memories)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn insert(&self, memory: &Memory) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO memories (id, agent_id, level, content, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                memory.id,
                memory.agent_id,
                memory.level,
                memory.content,
                memory.created_at,
            ],
        )?;
        Ok(())
    }
}
