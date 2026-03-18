use rusqlite::{Connection, params};

use crate::events::Events;
use crate::store::{StoreError, StoredEvent};

use super::events::*;
use super::model::Ticket;

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

        let result = stmt.query_row(params![id], |row| {
            Ok(Ticket {
                id: row.get(0)?,
                actor_id: row.get(1)?,
                brain_name: row.get(2)?,
                token: row.get(3)?,
                created_at: row.get(4)?,
            })
        });

        match result {
            Ok(ticket) => Ok(Some(ticket)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Ticket>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, actor_id, brain_name, token, created_at FROM tickets ORDER BY created_at",
        )?;

        let tickets = stmt
            .query_map([], |row| {
                Ok(Ticket {
                    id: row.get(0)?,
                    actor_id: row.get(1)?,
                    brain_name: row.get(2)?,
                    token: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tickets)
    }

    pub fn get_by_token(&self, token: &str) -> Result<Option<Ticket>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, actor_id, brain_name, token, created_at FROM tickets WHERE token = ?1",
        )?;

        let result = stmt.query_row(params![token], |row| {
            Ok(Ticket {
                id: row.get(0)?,
                actor_id: row.get(1)?,
                brain_name: row.get(2)?,
                token: row.get(3)?,
                created_at: row.get(4)?,
            })
        });

        match result {
            Ok(ticket) => Ok(Some(ticket)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, ticket: &Ticket) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO tickets (id, actor_id, brain_name, token, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                ticket.id,
                ticket.actor_id,
                ticket.brain_name,
                ticket.token,
                ticket.created_at
            ],
        )?;
        Ok(())
    }
}
