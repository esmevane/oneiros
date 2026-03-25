use crate::*;

pub struct TicketService;

impl TicketService {
    pub async fn create(
        context: &SystemContext,
        actor_id: ActorId,
        brain_name: BrainName,
    ) -> Result<TicketResponse, TicketError> {
        let db = context.db()?;

        // Look up the brain to get its ID for the token claims
        let brain = BrainRepo::new(&db)
            .get(&brain_name)?
            .ok_or_else(|| TicketError::BrainNotFound(brain_name.clone()))?;

        // Look up the actor to get its tenant_id for the token claims
        let actor = ActorRepo::new(&db)
            .get(&actor_id)?
            .ok_or_else(|| TicketError::ActorNotFound(actor_id))?;

        let claims = TokenClaims {
            brain_id: brain.id,
            tenant_id: actor.tenant_id,
            actor_id,
        };

        let token = Token::issue(claims);

        let ticket = Ticket {
            id: TicketId::new(),
            actor_id,
            brain_name,
            brain_id: brain.id,
            token,
            created_at: Timestamp::now(),
        };

        context
            .emit(TicketEvents::TicketIssued(ticket.clone()))
            .await?;
        Ok(TicketResponse::Created(ticket))
    }

    pub fn get(context: &SystemContext, id: &TicketId) -> Result<TicketResponse, TicketError> {
        let ticket = TicketRepo::new(&context.db()?)
            .get(id)?
            .ok_or_else(|| TicketError::NotFound(*id))?;
        Ok(TicketResponse::Found(ticket))
    }

    pub fn list(context: &SystemContext) -> Result<TicketResponse, TicketError> {
        let tickets = TicketRepo::new(&context.db()?).list()?;
        Ok(TicketResponse::Listed(tickets))
    }

    pub fn validate(context: &SystemContext, token: &str) -> Result<TicketResponse, TicketError> {
        let ticket = TicketRepo::new(&context.db()?)
            .get_by_token(token)?
            .ok_or(TicketError::InvalidToken)?;
        Ok(TicketResponse::Validated(ticket))
    }
}
