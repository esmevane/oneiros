use rusqlite::params;

use crate::*;

/// Connection read model — async queries over the projection read model.
pub struct ConnectionRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> ConnectionRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    // ── Read queries ────────────────────────────────────────────

    pub async fn get(&self, id: &ConnectionId) -> Result<Option<Connection>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare(
            "SELECT id, from_ref, to_ref, nature, created_at
             FROM connections WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        });

        match result {
            Ok((id, from_ref, to_ref, nature, created_at)) => Ok(Some(
                Connection::builder()
                    .id(id.parse()?)
                    .from_ref(serde_json::from_str(&from_ref)?)
                    .to_ref(serde_json::from_str(&to_ref)?)
                    .nature(nature)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list(&self, entity_ref: Option<&str>) -> Result<Vec<Connection>, EventError> {
        let db = self.context.db()?;
        let mut stmt = match entity_ref {
            Some(_) => db.prepare(
                "SELECT id, from_ref, to_ref, nature, created_at
                 FROM connections
                 WHERE from_ref = ?1 OR to_ref = ?1
                 ORDER BY created_at",
            )?,
            None => db.prepare(
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
}
