use rusqlite::params;

use crate::*;

fn is_missing_table(e: &rusqlite::Error) -> bool {
    matches!(
        e,
        rusqlite::Error::SqliteFailure(_, Some(msg))
            if msg.starts_with("no such table")
    )
}

pub struct BrainStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> BrainStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Brain(BrainEvents::BrainCreated(creation))) = &event.data {
            let brain = creation.current()?.brain;
            self.write_brain(&brain)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM brains", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create table if not exists brains (
                id text primary key,
                name text not null unique,
                created_at text not null default ''
            )",
        )?;
        Ok(())
    }

    /// List all brain names known to the system DB. Returns an empty
    /// list if the projection has not been migrated yet (cold start).
    pub fn list(&self) -> Result<Vec<BrainName>, rusqlite::Error> {
        let mut stmt = match self.conn.prepare("select name from brains") {
            Ok(stmt) => stmt,
            Err(e) if is_missing_table(&e) => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows.into_iter().map(BrainName::from).collect())
    }

    fn write_brain(&self, brain: &Brain) -> Result<(), EventError> {
        self.conn.execute(
            "insert or replace into brains (id, name, created_at) values (?1, ?2, ?3)",
            params![
                brain.id.to_string(),
                brain.name.to_string(),
                brain.created_at.to_string()
            ],
        )?;
        Ok(())
    }
}
