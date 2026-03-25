use rusqlite::params;

use crate::*;

pub struct TicketStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> TicketStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Ticket(TicketEvents::TicketIssued(ticket)) = &event.data {
            self.create_record(ticket)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM tickets", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tickets (
                id TEXT PRIMARY KEY,
                actor_id TEXT NOT NULL,
                brain_name TEXT NOT NULL,
                brain_id TEXT NOT NULL DEFAULT '',
                token TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    fn create_record(&self, ticket: &Ticket) -> Result<(), EventError> {
        self.conn.execute(
            "insert or replace into tickets (id, actor_id, brain_name, brain_id, token, created_at)
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                ticket.id.to_string(),
                ticket.actor_id.to_string(),
                ticket.brain_name.to_string(),
                ticket.brain_id.to_string(),
                ticket.token.as_str(),
                ticket.created_at.as_string()
            ],
        )?;
        Ok(())
    }
}
