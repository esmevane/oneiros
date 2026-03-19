use rusqlite::{Connection, params};

use crate::*;

/// Brain read model — queries, projection handling, and lifecycle.
pub struct BrainRepo<'a> {
    conn: &'a Connection,
}

impl<'a> BrainRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Brain(BrainEvents::BrainCreated(brain)) = &event.data {
            self.create_record(brain)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM brains", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS brains (
                name TEXT PRIMARY KEY,
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &str) -> Result<Option<Brain>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, created_at FROM brains WHERE name = ?1")?;

        let raw = stmt.query_row(params![name], |row| {
            let name: String = row.get(0)?;
            let created_at: String = row.get(1)?;
            Ok((name, created_at))
        });

        match raw {
            Ok((name, created_at)) => Ok(Some(Brain {
                name: BrainName::new(name),
                created_at,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Brain>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, created_at FROM brains ORDER BY name")?;

        let raw: Vec<(String, String)> = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut brains = vec![];

        for (name, created_at) in raw {
            brains.push(Brain {
                name: BrainName::new(name),
                created_at,
            });
        }

        Ok(brains)
    }

    pub fn name_exists(&self, name: &str) -> Result<bool, EventError> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM brains WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, brain: &Brain) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO brains (name, created_at) VALUES (?1, ?2)",
            params![brain.name.to_string(), brain.created_at],
        )?;
        Ok(())
    }
}
