use rusqlite::params;

use crate::*;

pub(crate) struct ActorStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> ActorStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Actor(ActorEvents::ActorCreated(actor)) = &event.data {
            self.create_record(actor)?;
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("delete from actors", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create table if not exists actors (
                id text primary key,
                tenant_id text not null,
                name text not null,
                created_at text not null default ''
            )",
        )?;
        Ok(())
    }

    fn create_record(&self, actor: &Actor) -> Result<(), EventError> {
        self.conn.execute(
            "insert or replace into actors (id, tenant_id, name, created_at)
             values (?1, ?2, ?3, ?4)",
            params![
                actor.id.to_string(),
                actor.tenant_id.to_string(),
                actor.name.to_string(),
                actor.created_at.as_string()
            ],
        )?;
        Ok(())
    }
}
