use chrono::Utc;
use uuid::Uuid;

use crate::*;

pub struct TicketService;

impl TicketService {
    pub async fn create(
        context: &SystemContext,
        actor_id: ActorId,
        brain_name: BrainName,
    ) -> Result<TicketResponse, TicketError> {
        let ticket = Ticket {
            id: TicketId::new(),
            actor_id,
            brain_name: brain_name.to_string(),
            token: Uuid::now_v7().to_string(),
            created_at: Utc::now().to_rfc3339(),
        };

        context
            .emit(TicketEvents::TicketIssued(ticket.clone()))
            .await?;
        Ok(TicketResponse::Created(ticket))
    }

    pub fn get(context: &SystemContext, id: &TicketId) -> Result<TicketResponse, TicketError> {
        let ticket = context
            .with_db(|conn| TicketRepo::new(conn).get(id))?
            .ok_or_else(|| TicketError::NotFound(*id))?;
        Ok(TicketResponse::Found(ticket))
    }

    pub fn list(context: &SystemContext) -> Result<TicketResponse, TicketError> {
        let tickets = context
            .with_db(|conn| TicketRepo::new(conn).list())
            .map_err(TicketError::Database)?;
        Ok(TicketResponse::Listed(tickets))
    }

    pub fn validate(context: &SystemContext, token: &str) -> Result<TicketResponse, TicketError> {
        let ticket = context
            .with_db(|conn| TicketRepo::new(conn).get_by_token(token))
            .map_err(TicketError::Database)?
            .ok_or(TicketError::InvalidToken)?;
        Ok(TicketResponse::Validated(ticket))
    }
}
