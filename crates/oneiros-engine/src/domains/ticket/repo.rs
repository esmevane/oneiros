use rusqlite::params;

use crate::*;

/// Ticket read model — queries, projection handling, and lifecycle.
pub struct TicketRepo<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> TicketRepo<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

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

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &TicketId) -> Result<Option<Ticket>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, actor_id, brain_name, brain_id, token, created_at FROM tickets WHERE id = ?1",
        )?;

        let raw: Result<(String, String, String, String, String, String), _> =
            stmt.query_row(params![id.to_string()], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            });

        match raw {
            Ok((id, actor_id, brain_name, brain_id, token, created_at)) => Ok(Some(Ticket {
                id: id.parse()?,
                actor_id: actor_id.parse()?,
                brain_name: BrainName::new(brain_name),
                brain_id: brain_id.parse()?,
                token: Token::from(token),
                created_at: Timestamp::parse_str(created_at)?,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Ticket>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, actor_id, brain_name, brain_id, token, created_at FROM tickets ORDER BY created_at",
        )?;

        let raw: Vec<(String, String, String, String, String, String)> = stmt
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut tickets = vec![];

        for (id, actor_id, brain_name, brain_id, token, created_at) in raw {
            tickets.push(Ticket {
                id: id.parse()?,
                actor_id: actor_id.parse()?,
                brain_name: BrainName::new(brain_name),
                brain_id: brain_id.parse()?,
                token: Token::from(token),
                created_at: Timestamp::parse_str(created_at)?,
            });
        }

        Ok(tickets)
    }

    pub fn get_by_token(&self, token: &str) -> Result<Option<Ticket>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, actor_id, brain_name, brain_id, token, created_at FROM tickets WHERE token = ?1",
        )?;

        let raw: Result<(String, String, String, String, String, String), _> =
            stmt.query_row(params![token], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            });

        match raw {
            Ok((id, actor_id, brain_name, brain_id, token, created_at)) => Ok(Some(Ticket {
                id: id.parse()?,
                actor_id: actor_id.parse()?,
                brain_name: BrainName::new(brain_name),
                brain_id: brain_id.parse()?,
                token: Token::from(token),
                created_at: Timestamp::parse_str(created_at)?,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // ── Write operations (called by handle) ─────────────────────

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
