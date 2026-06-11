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
        if let Event::Known(Events::Peer(peer_event)) = &event.data {
            match peer_event {
                PeerEvents::PeerAdded(added) => {
                    let current = added.current()?;
                    self.create_record(
                        &current.id,
                        &current.key,
                        &current.address,
                        &current.name,
                        current.kind,
                        current.ticket.as_ref(),
                        current.project.as_ref(),
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
                        current.kind,
                        current.ticket.as_ref(),
                        current.project.as_ref(),
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
                kind text not null default 'bookmark',
                ticket_token text,
                ticket_target text,
                project text,
                created_at text not null default ''
            )",
        )?;
        Ok(())
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "This actually does need trimming but we'll get to it as part of a structural refactor"
    )]
    fn create_record(
        &self,
        id: &PeerId,
        key: &PeerKey,
        address: &PeerAddress,
        name: &PeerName,
        kind: PeerKind,
        ticket: Option<&Link>,
        project: Option<&ProjectName>,
        created_at: &Timestamp,
    ) -> Result<(), EventError> {
        let (ticket_token, ticket_target) = match ticket {
            Some(link) => (
                link.token.to_string(),
                RefToken::from(link.target.clone()).to_string(),
            ),
            None => (String::new(), String::new()),
        };
        let project_str = project.map(|p| p.to_string()).unwrap_or_default();
        self.conn.execute(
            "insert or replace into peers (id, key, address, name, kind, ticket_token, ticket_target, project, created_at)
             values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                id.to_string(),
                key.to_string(),
                address.to_string(),
                name.to_string(),
                kind.to_string(),
                ticket_token,
                ticket_target,
                project_str,
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
