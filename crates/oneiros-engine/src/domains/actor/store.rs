use rusqlite::params;

use crate::*;

pub struct ActorStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> ActorStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Actor(ActorEvents::ActorCreated(creation))) = &event.data {
            let actor = creation.current()?.actor;
            self.write_actor(&actor)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("delete from actors", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
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

    fn write_actor(&self, actor: &Actor) -> Result<(), EventError> {
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
