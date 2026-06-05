use rusqlite::params;

use crate::*;

pub(crate) struct RemoteStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> RemoteStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Remote(remote_event)) = &event.data {
            match remote_event {
                RemoteEvents::RemoteAdded(added) => {
                    let current = added.current()?;
                    self.create_record(&current.remote)?;
                }
                RemoteEvents::RemoteRemoved(removed) => {
                    let current = removed.current()?;
                    self.delete_record(current.id)?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM remotes", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS remotes (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                address TEXT NOT NULL,
                ticket_token TEXT NOT NULL,
                ticket_target TEXT NOT NULL,
                project_name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    fn create_record(&self, remote: &Remote) -> Result<(), EventError> {
        let target = RefToken::new(remote.ticket.target.clone()).to_string();
        self.conn.execute(
            "INSERT OR REPLACE INTO remotes (
                id, name, address, ticket_token, ticket_target, project_name, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                remote.id.to_string(),
                remote.name.to_string(),
                remote.address.to_string(),
                remote.ticket.token.as_str(),
                target,
                remote.project.to_string(),
                remote.created_at.as_string(),
            ],
        )?;
        Ok(())
    }

    fn delete_record(&self, id: RemoteId) -> Result<(), EventError> {
        self.conn
            .execute("DELETE FROM remotes WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
