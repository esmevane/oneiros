use rusqlite::params;

use crate::*;

/// Connection projection store — projection lifecycle, write operations, and sync read queries.
pub(crate) struct ConnectionStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> ConnectionStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Connection(connection_event) = &event.data {
            match connection_event {
                ConnectionEvents::ConnectionCreated(connection) => self.insert(connection)?,
                ConnectionEvents::ConnectionRemoved(removed) => self.remove(&removed.id)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM connections", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS connections (
                id TEXT PRIMARY KEY,
                from_ref TEXT NOT NULL,
                to_ref TEXT NOT NULL,
                nature TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    pub(crate) fn list(&self, entity_ref: Option<&str>) -> Result<Vec<Connection>, EventError> {
        let mut stmt = match entity_ref {
            Some(_) => self.conn.prepare(
                "SELECT id, from_ref, to_ref, nature, created_at
                 FROM connections
                 WHERE from_ref = ?1 OR to_ref = ?1
                 ORDER BY created_at",
            )?,
            None => self.conn.prepare(
                "SELECT id, from_ref, to_ref, nature, created_at
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
            ))
        };

        let raw = match entity_ref {
            Some(e) => stmt.query_map(params![e], map_row),
            None => stmt.query_map([], map_row),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        let mut connections = vec![];
        for (id, from_ref, to_ref, nature, created_at) in raw {
            connections.push(
                Connection::builder()
                    .id(id.parse()?)
                    .from_ref(serde_json::from_str(&from_ref)?)
                    .to_ref(serde_json::from_str(&to_ref)?)
                    .nature(nature)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            );
        }

        Ok(connections)
    }

    fn insert(&self, connection: &Connection) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO connections
             (id, from_ref, to_ref, nature, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                connection.id.to_string(),
                serde_json::to_string(&connection.from_ref)?,
                serde_json::to_string(&connection.to_ref)?,
                connection.nature.to_string(),
                connection.created_at.as_string(),
            ],
        )?;
        Ok(())
    }

    fn remove(&self, id: &ConnectionId) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM connections WHERE id = ?1",
            params![id.to_string()],
        )?;
        Ok(())
    }
}
