use rusqlite::{Connection as DbConn, params};

use crate::*;

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
        if let Events::Connection(connection_event) = &event.data {
            match connection_event {
                ConnectionEvents::ConnectionCreated(connection) => self.insert(connection)?,
                ConnectionEvents::ConnectionRemoved(removed) => {
                    self.remove(&removed.id.to_string())?
                }
            }
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
            let id: String = row.get(0)?;
            Ok((id, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?, row.get::<_, String>(4)?, row.get::<_, String>(5)?))
        });

        match result {
            Ok((id, from_entity, to_entity, nature, description, created_at)) => {
                Ok(Some(Connection {
                    id: id.parse()?,
                    from_entity,
                    to_entity,
                    nature,
                    description,
                    created_at,
                }))
            }
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
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        };

        let raw = match entity {
            Some(e) => stmt.query_map(params![e], map_row),
            None => stmt.query_map([], map_row),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        let mut connections = vec![];
        for (id, from_entity, to_entity, nature, description, created_at) in raw {
            connections.push(Connection {
                id: id.parse()?,
                from_entity,
                to_entity,
                nature,
                description,
                created_at,
            });
        }

        Ok(connections)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn insert(&self, connection: &Connection) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO connections
             (id, from_entity, to_entity, nature, description, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                connection.id.to_string(),
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
