use crate::*;

pub struct TicketService;

impl TicketService {
    pub async fn create(
        context: &SystemContext,
        actor_id: ActorId,
        brain_name: BrainName,
    ) -> Result<TicketResponse, TicketError> {
        // Look up the brain to get its ID for the token claims
        let brain = BrainRepo::new(context)
            .get(&brain_name)
            .await?
            .ok_or_else(|| TicketError::BrainNotFound(brain_name.clone()))?;

        // Look up the actor to get its tenant_id for the token claims
        let actor = ActorRepo::new(context)
            .get(actor_id)
            .await?
            .ok_or_else(|| TicketError::ActorNotFound(actor_id))?;

        let claims = TokenClaims::builder()
            .brain_id(brain.id)
            .tenant_id(actor.tenant_id)
            .actor_id(actor_id)
            .build();

        let token = Token::issue(claims);
        let ticket = Ticket::builder()
            .actor_id(actor_id)
            .brain_name(brain_name)
            .brain_id(brain.id)
            .token(token)
            .build();

        context
            .emit(TicketEvents::TicketIssued(ticket.clone()))
            .await?;
        Ok(TicketResponse::Created(ticket))
    }

    pub async fn get(
        context: &SystemContext,
        id: &TicketId,
    ) -> Result<TicketResponse, TicketError> {
        let ticket = TicketRepo::new(context)
            .get(id)
            .await?
            .ok_or_else(|| TicketError::NotFound(*id))?;
        Ok(TicketResponse::Found(ticket))
    }

    pub async fn list(context: &SystemContext) -> Result<TicketResponse, TicketError> {
        let tickets = TicketRepo::new(context).list().await?;
        Ok(TicketResponse::Listed(tickets))
    }

    pub async fn validate(
        context: &SystemContext,
        token: &str,
    ) -> Result<TicketResponse, TicketError> {
        let ticket = TicketRepo::new(context)
            .get_by_token(token)
            .await?
            .ok_or(TicketError::InvalidToken)?;
        Ok(TicketResponse::Validated(ticket))
    }
}
