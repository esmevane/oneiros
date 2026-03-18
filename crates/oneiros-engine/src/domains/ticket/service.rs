use chrono::Utc;
use uuid::Uuid;

use crate::contexts::SystemContext;

use super::errors::TicketError;
use super::model::Ticket;
use super::repo::TicketRepo;
use super::responses::TicketResponse;

pub struct TicketService;

impl TicketService {
    pub fn create(
        ctx: &SystemContext,
        actor_id: String,
        brain_name: String,
    ) -> Result<TicketResponse, TicketError> {
        let ticket = Ticket {
            id: Uuid::now_v7().to_string(),
            actor_id,
            brain_name,
            token: Uuid::now_v7().to_string(),
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit("ticket-created", &ticket);
        Ok(TicketResponse::Created(ticket))
    }

    pub fn get(ctx: &SystemContext, id: &str) -> Result<TicketResponse, TicketError> {
        let ticket = ctx
            .with_db(|conn| TicketRepo::new(conn).get(id))
            .map_err(TicketError::Database)?
            .ok_or_else(|| TicketError::NotFound(id.to_string()))?;
        Ok(TicketResponse::Found(ticket))
    }

    pub fn list(ctx: &SystemContext) -> Result<TicketResponse, TicketError> {
        let tickets = ctx
            .with_db(|conn| TicketRepo::new(conn).list())
            .map_err(TicketError::Database)?;
        Ok(TicketResponse::Listed(tickets))
    }

    pub fn validate(ctx: &SystemContext, token: &str) -> Result<TicketResponse, TicketError> {
        let ticket = ctx
            .with_db(|conn| TicketRepo::new(conn).get_by_token(token))
            .map_err(TicketError::Database)?
            .ok_or(TicketError::InvalidToken)?;
        Ok(TicketResponse::Validated(ticket))
    }
}
