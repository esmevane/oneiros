use rusqlite::{Connection, params};

use crate::store::{StoredEvent, StoreError};

use super::model::Actor;

/// Actor read model — queries, projection handling, and lifecycle.
pub struct ActorRepo<'a> {
    conn: &'a Connection,
}

impl<'a> ActorRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if event.event_type == "actor-created" {
            let actor: Actor = serde_json::from_value(event.data.clone())?;
            self.create_record(&actor)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM actors", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS actors (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &str) -> Result<Option<Actor>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, tenant_id, name, created_at FROM actors WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id], |row| {
            Ok(Actor {
                id: row.get(0)?,
                tenant_id: row.get(1)?,
                name: row.get(2)?,
                created_at: row.get(3)?,
            })
        });

        match result {
            Ok(actor) => Ok(Some(actor)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Actor>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, tenant_id, name, created_at FROM actors ORDER BY name",
        )?;

        let actors = stmt
            .query_map([], |row| {
                Ok(Actor {
                    id: row.get(0)?,
                    tenant_id: row.get(1)?,
                    name: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(actors)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, actor: &Actor) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO actors (id, tenant_id, name, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![actor.id, actor.tenant_id, actor.name, actor.created_at],
        )?;
        Ok(())
    }
}
