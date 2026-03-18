use rusqlite::{Connection, params};

use crate::*;

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
        if let Events::Actor(ActorEvents::ActorCreated(actor)) = &event.data {
            self.create_record(actor)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("delete from actors", [])?;
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
        let mut stmt = self
            .conn
            .prepare("SELECT id, tenant_id, name, created_at FROM actors WHERE id = ?1")?;

        let raw = stmt.query_row(params![id], |row| {
            let id: String = row.get(0)?;
            let tenant_id: String = row.get(1)?;
            let name: String = row.get(2)?;
            let created_at: String = row.get(3)?;

            Ok((id, tenant_id, name, created_at))
        });

        match raw {
            Ok((id, tenant_id, name, created_at)) => Ok(Some(Actor {
                id: id.parse()?,
                tenant_id,
                name: ActorName::new(name),
                created_at,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Actor>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("select id, tenant_id, name, created_at from actors order by name")?;

        let raw: Vec<(String, String, String, String)> = stmt
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut actors = vec![];

        for (id, tenant_id, name, created_at) in raw {
            actors.push(Actor {
                id: id.parse()?,
                tenant_id,
                name: ActorName::new(name),
                created_at,
            });
        }

        Ok(actors)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, actor: &Actor) -> Result<(), StoreError> {
        self.conn.execute(
            "insert or replace into actors (id, tenant_id, name, created_at)
             values (?1, ?2, ?3, ?4)",
            params![
                actor.id.to_string(),
                actor.tenant_id,
                actor.name.to_string(),
                actor.created_at
            ],
        )?;
        Ok(())
    }
}
