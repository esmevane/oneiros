use rusqlite::params;

use crate::*;

pub(crate) struct TicketStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> TicketStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Ticket(TicketEvents::TicketIssued(issued))) = &event.data {
            let ticket = issued.current()?.ticket;
            self.write_ticket(&ticket)?;
        } else if let Event::Known(Events::Ticket(TicketEvents::TicketRevoked(revoked))) =
            &event.data
        {
            let current = revoked.current()?;
            self.conn.execute(
                "UPDATE tickets SET revoked_at = ?1 WHERE id = ?2",
                rusqlite::params![
                    current.revoked_at.as_string(),
                    current.ticket_id.to_string()
                ],
            )?;
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM tickets", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tickets (
                id TEXT PRIMARY KEY,
                actor_id TEXT NOT NULL,
                project_name TEXT NOT NULL,
                project_id TEXT NOT NULL DEFAULT '',
                token TEXT NOT NULL UNIQUE,
                target TEXT NOT NULL DEFAULT '',
                granted_by TEXT NOT NULL DEFAULT '',
                expires_at TEXT,
                revoked_at TEXT,
                max_uses INTEGER,
                uses INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT '',
                permissions TEXT NOT NULL DEFAULT '[]'
            )",
        )?;
        // Migration: add column if missing from older schemas.
        self.conn
            .execute_batch("ALTER TABLE tickets ADD COLUMN permissions TEXT NOT NULL DEFAULT '[]'")
            .ok();
        Ok(())
    }

    fn write_ticket(&self, ticket: &Ticket) -> Result<(), EventError> {
        let target = RefToken::new(ticket.link.target.clone()).to_string();
        let permissions_json =
            serde_json::to_string(&ticket.permissions).unwrap_or_else(|_| "[]".into());
        self.conn.execute(
            "insert or replace into tickets (
                id, actor_id, project_name, project_id, token, target, granted_by,
                expires_at, revoked_at, max_uses, uses, created_at, permissions
             )
             values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                ticket.id.to_string(),
                ticket.actor_id.to_string(),
                ticket.project_name.to_string(),
                ticket.project_id.to_string(),
                ticket.link.token.as_str(),
                target,
                ticket.granted_by.to_string(),
                ticket.expires_at.map(|t| t.as_string()),
                ticket.revoked_at.map(|t| t.as_string()),
                ticket.max_uses,
                ticket.uses,
                ticket.created_at.as_string(),
                permissions_json,
            ],
        )?;
        Ok(())
    }
}
