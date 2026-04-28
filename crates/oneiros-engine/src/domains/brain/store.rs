use rusqlite::params;

use crate::*;

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
