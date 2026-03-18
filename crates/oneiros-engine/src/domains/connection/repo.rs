use rusqlite::{Connection as DbConn, params};

use crate::store::{StoreError, StoredEvent};

use super::model::Connection;

/// Connection read model — queries, projection handling, and lifecycle.
pub struct ConnectionRepo<'a> {
    conn: &'a DbConn,
}

impl<'a> ConnectionRepo<'a> {
    pub fn new(conn: &'a DbConn) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        match event.event_type.as_str() {
            "connection-created" => {
                let connection: Connection = serde_json::from_value(event.data.clone())?;
                self.insert(&connection)?;
            }
            "connection-removed" => {
                if let Some(id) = event.data.get("id").and_then(|v| v.as_str()) {
                    self.remove(id)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM connections", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS connections (
                id TEXT PRIMARY KEY,
                from_entity TEXT NOT NULL,
                to_entity TEXT NOT NULL,
                nature TEXT NOT NULL DEFAULT '',
                description TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &str) -> Result<Option<Connection>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, from_entity, to_entity, nature, description, created_at
             FROM connections WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id], |row| {
            Ok(Connection {
                id: row.get(0)?,
                from_entity: row.get(1)?,
                to_entity: row.get(2)?,
                nature: row.get(3)?,
                description: row.get(4)?,
                created_at: row.get(5)?,
            })
        });

        match result {
            Ok(c) => Ok(Some(c)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self, entity: Option<&str>) -> Result<Vec<Connection>, StoreError> {
        let mut stmt = match entity {
            Some(_) => self.conn.prepare(
                "SELECT id, from_entity, to_entity, nature, description, created_at
                 FROM connections
                 WHERE from_entity = ?1 OR to_entity = ?1
                 ORDER BY created_at",
            )?,
            None => self.conn.prepare(
                "SELECT id, from_entity, to_entity, nature, description, created_at
                 FROM connections ORDER BY created_at",
            )?,
        };

        let map_row = |row: &rusqlite::Row<'_>| {
            Ok(Connection {
                id: row.get(0)?,
                from_entity: row.get(1)?,
                to_entity: row.get(2)?,
                nature: row.get(3)?,
                description: row.get(4)?,
                created_at: row.get(5)?,
            })
        };

        let connections = match entity {
            Some(e) => stmt.query_map(params![e], map_row),
            None => stmt.query_map([], map_row),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(connections)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn insert(&self, connection: &Connection) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO connections
             (id, from_entity, to_entity, nature, description, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                connection.id,
                connection.from_entity,
                connection.to_entity,
                connection.nature,
                connection.description,
                connection.created_at,
            ],
        )?;
        Ok(())
    }

    fn remove(&self, id: &str) -> Result<(), StoreError> {
        self.conn
            .execute("DELETE FROM connections WHERE id = ?1", params![id])?;
        Ok(())
    }
}
