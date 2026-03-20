use rusqlite::{Connection, params};

use crate::*;

/// Nature read model — queries, projection handling, and lifecycle.
pub struct NatureRepo<'a> {
    conn: &'a Connection,
}

impl<'a> NatureRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Nature(nature_event) = &event.data {
            match nature_event {
                NatureEvents::NatureSet(nature) => self.set(nature)?,
                NatureEvents::NatureRemoved(removed) => self.remove(&removed.name)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM natures", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS natures (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &NatureName) -> Result<Option<Nature>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM natures WHERE name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok((name, description, prompt)) => Ok(Some(
                Nature::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Nature>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM natures ORDER BY name")?;

        let natures = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })?
            .collect::<Result<Vec<(String, String, String)>, _>>()?
            .into_iter()
            .map(|(name, description, prompt)| {
                Nature::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(natures)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, nature: &Nature) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO natures (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                nature.name.to_string(),
                nature.description.to_string(),
                nature.prompt.to_string()
            ],
        )?;
        Ok(())
    }

    fn remove(&self, name: &NatureName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM natures WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }
}
