use rusqlite::params;

use crate::*;

/// Brain read model — queries, projection handling, and lifecycle.
pub struct BrainRepo<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> BrainRepo<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
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
            "create table if not exists brains (
                id text primary key,
                name text not null unique,
                created_at text not null default ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &BrainName) -> Result<Option<Brain>, EventError> {
        let mut stmt = self
            .conn
            .prepare("select id, name, created_at from brains where name = ?1")?;

        let raw = stmt.query_row(params![name.to_string()], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok((id, name, created_at))
        });

        match raw {
            Ok((id, name, created_at)) => Ok(Some(
                Brain::builder()
                    .id(id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_by_id(&self, id: &BrainId) -> Result<Option<Brain>, EventError> {
        let mut stmt = self
            .conn
            .prepare("select id, name, created_at from brains where id = ?1")?;

        let raw = stmt.query_row(params![id.to_string()], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok((id, name, created_at))
        });

        match raw {
            Ok((id, name, created_at)) => Ok(Some(
                Brain::builder()
                    .id(id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Brain>, EventError> {
        let mut stmt = self
            .conn
            .prepare("select id, name, created_at from brains order by name")?;

        let raw: Vec<(String, String, String)> = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut brains = vec![];

        for (id, name, created_at) in raw {
            brains.push(
                Brain::builder()
                    .id(id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            );
        }

        Ok(brains)
    }

    pub fn name_exists(&self, name: &BrainName) -> Result<bool, EventError> {
        let count: i64 = self.conn.query_row(
            "select count(*) from brains where name = ?1",
            params![name.to_string()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, brain: &Brain) -> Result<(), EventError> {
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
