use rusqlite::{Connection, params};

use crate::store::{StoreError, StoredEvent};

use super::model::Brain;

/// Brain read model — queries, projection handling, and lifecycle.
pub struct BrainRepo<'a> {
    conn: &'a Connection,
}

impl<'a> BrainRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if event.event_type == "brain-created" {
            let brain: Brain = serde_json::from_value(event.data.clone())?;
            self.create_record(&brain)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM brains", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS brains (
                name TEXT PRIMARY KEY,
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &str) -> Result<Option<Brain>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, created_at FROM brains WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            Ok(Brain {
                name: row.get(0)?,
                created_at: row.get(1)?,
            })
        });

        match result {
            Ok(brain) => Ok(Some(brain)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Brain>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, created_at FROM brains ORDER BY name")?;

        let brains = stmt
            .query_map([], |row| {
                Ok(Brain {
                    name: row.get(0)?,
                    created_at: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(brains)
    }

    pub fn name_exists(&self, name: &str) -> Result<bool, StoreError> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM brains WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, brain: &Brain) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO brains (name, created_at) VALUES (?1, ?2)",
            params![brain.name, brain.created_at],
        )?;
        Ok(())
    }
}
