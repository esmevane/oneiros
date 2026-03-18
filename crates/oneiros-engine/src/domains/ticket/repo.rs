use rusqlite::{Connection, params};

use crate::*;

/// Ticket read model — queries, projection handling, and lifecycle.
pub struct TicketRepo<'a> {
    conn: &'a Connection,
}

impl<'a> TicketRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if let Events::Ticket(TicketEvents::TicketIssued(ticket)) = &event.data {
            self.create_record(ticket)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM tickets", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tickets (
                id TEXT PRIMARY KEY,
                actor_id TEXT NOT NULL,
                brain_name TEXT NOT NULL,
                token TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &str) -> Result<Option<Ticket>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, actor_id, brain_name, token, created_at FROM tickets WHERE id = ?1",
        )?;

        let raw: Result<(String, String, String, String, String), _> =
            stmt.query_row(params![id], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            });

        match raw {
            Ok((id, actor_id, brain_name, token, created_at)) => Ok(Some(Ticket {
                id: id.parse()?,
                actor_id: actor_id.parse()?,
                brain_name,
                token,
                created_at,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Ticket>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, actor_id, brain_name, token, created_at FROM tickets ORDER BY created_at",
        )?;

        let raw: Vec<(String, String, String, String, String)> = stmt
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut tickets = vec![];

        for (id, actor_id, brain_name, token, created_at) in raw {
            tickets.push(Ticket {
                id: id.parse()?,
                actor_id: actor_id.parse()?,
                brain_name,
                token,
                created_at,
            });
        }

        Ok(tickets)
    }

    pub fn get_by_token(&self, token: &str) -> Result<Option<Ticket>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, actor_id, brain_name, token, created_at FROM tickets WHERE token = ?1",
        )?;

        let raw: Result<(String, String, String, String, String), _> =
            stmt.query_row(params![token], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            });

        match raw {
            Ok((id, actor_id, brain_name, token, created_at)) => Ok(Some(Ticket {
                id: id.parse()?,
                actor_id: actor_id.parse()?,
                brain_name,
                token,
                created_at,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, ticket: &Ticket) -> Result<(), StoreError> {
        self.conn.execute(
            "insert or replace into tickets (id, actor_id, brain_name, token, created_at)
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                ticket.id.to_string(),
                ticket.actor_id.to_string(),
                ticket.brain_name,
                ticket.token,
                ticket.created_at
            ],
        )?;
        Ok(())
    }
}
