use rusqlite::params;

use crate::*;

pub struct PeerStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> PeerStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Peer(peer_event)) = &event.data {
            match peer_event {
                PeerEvents::PeerAdded(added) => {
                    let current = added.current()?;
                    self.create_record(
                        &current.id,
                        &current.key,
                        &current.address,
                        &current.name,
                        &current.created_at,
                    )?;
                }
                PeerEvents::PeerUpdated(updated) => {
                    let current = updated.current()?;
                    self.create_record(
                        &current.id,
                        &current.key,
                        &current.address,
                        &current.name,
                        &current.created_at,
                    )?;
                }
                PeerEvents::PeerRemoved(removed) => {
                    let current = removed.current()?;
                    self.delete_record(current.id)?;
                }
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("delete from peers", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create table if not exists peers (
                id text primary key,
                key text not null unique,
                address text not null,
                name text not null,
                created_at text not null default ''
            )",
        )?;
        Ok(())
    }

    fn create_record(
        &self,
        id: &PeerId,
        key: &PeerKey,
        address: &PeerAddress,
        name: &PeerName,
        created_at: &Timestamp,
    ) -> Result<(), EventError> {
        self.conn.execute(
            "insert or replace into peers (id, key, address, name, created_at)
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                id.to_string(),
                key.to_string(),
                address.to_string(),
                name.to_string(),
                created_at.as_string(),
            ],
        )?;
        Ok(())
    }

    fn delete_record(&self, id: PeerId) -> Result<(), EventError> {
        self.conn
            .execute("delete from peers where id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
