use rusqlite::params;

use crate::*;

/// Ticket read model — async queries against the system context.
pub struct TicketRepo<'a> {
    context: &'a SystemContext,
}

impl<'a> TicketRepo<'a> {
    pub fn new(context: &'a SystemContext) -> Self {
        Self { context }
    }

    pub async fn get(&self, id: &TicketId) -> Result<Option<Ticket>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare(
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

    pub async fn list(&self, filters: &SearchFilters) -> Result<Listed<Ticket>, EventError> {
        let db = self.context.db()?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM tickets")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let mut stmt = db.prepare(
            "SELECT id, actor_id, brain_name, brain_id, token, created_at
             FROM tickets
             ORDER BY created_at DESC
             LIMIT ?1 OFFSET ?2",
        )?;

        let raw: Vec<(String, String, String, String, String, String)> = stmt
            .query_map(rusqlite::params![filters.limit, filters.offset], |row| {
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

        Ok(Listed::new(tickets, total))
    }

    pub async fn get_by_token(&self, token: &str) -> Result<Option<Ticket>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare(
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
}
