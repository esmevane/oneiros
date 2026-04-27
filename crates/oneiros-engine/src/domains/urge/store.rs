use rusqlite::params;

use crate::*;

pub struct UrgeStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> UrgeStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Urge(urge_event)) = &event.data {
            match urge_event {
                UrgeEvents::UrgeSet(urge) => self.set(urge)?,
                UrgeEvents::UrgeRemoved(removed) => self.remove(removed.name())?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM urges", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS urges (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Urge>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM urges ORDER BY name")?;

        let urges = stmt
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
                Urge::Current(
                    Urge::build_v1()
                        .name(name)
                        .description(description)
                        .prompt(prompt)
                        .build(),
                )
            })
            .collect();

        Ok(urges)
    }

    fn set(&self, urge: &Urge) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO urges (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                urge.name().to_string(),
                urge.description().to_string(),
                urge.prompt().to_string()
            ],
        )?;
        Ok(())
    }

    fn remove(&self, name: &UrgeName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM urges WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }
}
