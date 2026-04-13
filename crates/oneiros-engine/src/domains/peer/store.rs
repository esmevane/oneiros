use rusqlite::params;

use crate::*;

pub(crate) struct PeerStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> PeerStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        match &event.data {
            Events::Peer(PeerEvents::PeerAdded(peer)) => {
                self.create_record(peer)?;
            }
            Events::Peer(PeerEvents::PeerUpdated(peer)) => {
                self.create_record(peer)?;
            }
            Events::Peer(PeerEvents::PeerRemoved(removed)) => {
                self.delete_record(removed.id)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("delete from peers", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
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

    fn create_record(&self, peer: &Peer) -> Result<(), EventError> {
        self.conn.execute(
            "insert or replace into peers (id, key, address, name, created_at)
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                peer.id.to_string(),
                peer.key.to_string(),
                peer.address.to_string(),
                peer.name.to_string(),
                peer.created_at.as_string(),
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
