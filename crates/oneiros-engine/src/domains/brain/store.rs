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
