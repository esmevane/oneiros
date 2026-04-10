use rusqlite::params;

use crate::*;

/// Ticket read model — async queries against the system context.
pub struct TicketRepo<'a> {
    context: &'a SystemContext,
}

/// The full projected row for a ticket.
type TicketRow = (
    String,         // id
    String,         // actor_id
    String,         // brain_name
    String,         // brain_id
    String,         // token
    String,         // target (RefToken string form)
    String,         // granted_by
    Option<String>, // expires_at
    Option<String>, // revoked_at
    Option<i64>,    // max_uses
    i64,            // uses
    String,         // created_at
);

const SELECT_COLUMNS: &str = "id, actor_id, brain_name, brain_id, token, target, granted_by, \
                              expires_at, revoked_at, max_uses, uses, created_at";

fn ticket_from_row(row: TicketRow) -> Result<Ticket, EventError> {
    let (
        id,
        actor_id,
        brain_name,
        brain_id,
        token,
        target,
        granted_by,
        expires_at,
        revoked_at,
        max_uses,
        uses,
        created_at,
    ) = row;

    let target_ref: RefToken = target
        .parse()
        .map_err(|e: RefError| EventError::Import(e.to_string()))?;
    let link = Link::new(target_ref.into_inner(), Token::from(token));

    let expires_at = expires_at.map(Timestamp::parse_str).transpose()?;
    let revoked_at = revoked_at.map(Timestamp::parse_str).transpose()?;

    // Legacy rows may predate `granted_by`; fall back to actor_id when
    // the column is empty to keep auth semantics stable until the row is
    // re-emitted.
    let actor_id: ActorId = actor_id.parse()?;
    let granted_by_id: ActorId = if granted_by.is_empty() {
        actor_id
    } else {
        granted_by.parse()?
    };

    Ok(Ticket {
        id: id.parse()?,
        actor_id,
        brain_name: BrainName::new(brain_name),
        brain_id: brain_id.parse()?,
        link,
        granted_by: granted_by_id,
        expires_at,
        revoked_at,
        max_uses: max_uses.map(|v| v as u64),
        uses: uses as u64,
        created_at: Timestamp::parse_str(created_at)?,
    })
}

fn read_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TicketRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
        row.get(10)?,
        row.get(11)?,
    ))
}

impl<'a> TicketRepo<'a> {
    pub fn new(context: &'a SystemContext) -> Self {
        Self { context }
    }

    pub async fn get(&self, id: &TicketId) -> Result<Option<Ticket>, EventError> {
        let db = self.context.db()?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM tickets WHERE id = ?1");
        let mut stmt = db.prepare(&sql)?;

        let raw: Result<TicketRow, _> = stmt.query_row(params![id.to_string()], read_row);

        match raw {
            Ok(row) => Ok(Some(ticket_from_row(row)?)),
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

        let sql = format!(
            "SELECT {SELECT_COLUMNS}
             FROM tickets
             ORDER BY created_at DESC
             LIMIT ?1 OFFSET ?2"
        );
        let mut stmt = db.prepare(&sql)?;

        let raw: Vec<TicketRow> = stmt
            .query_map(rusqlite::params![filters.limit, filters.offset], read_row)?
            .collect::<Result<Vec<_>, _>>()?;

        let mut tickets = vec![];

        for row in raw {
            tickets.push(ticket_from_row(row)?);
        }

        Ok(Listed::new(tickets, total))
    }

    pub async fn get_by_token(&self, token: &str) -> Result<Option<Ticket>, EventError> {
        let db = self.context.db()?;
        let sql = format!("SELECT {SELECT_COLUMNS} FROM tickets WHERE token = ?1");
        let mut stmt = db.prepare(&sql)?;

        let raw: Result<TicketRow, _> = stmt.query_row(params![token], read_row);

        match raw {
            Ok(row) => Ok(Some(ticket_from_row(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
