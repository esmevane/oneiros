use crate::*;

pub struct TicketService;

impl TicketService {
    pub async fn create(
        context: &SystemContext,
        CreateTicket {
            actor_id,
            brain_name,
        }: &CreateTicket,
    ) -> Result<TicketResponse, TicketError> {
        // Look up the brain to get its ID for the token claims
        let brain = BrainRepo::new(context)
            .get(brain_name)
            .await?
            .ok_or_else(|| TicketError::BrainNotFound(brain_name.clone()))?;

        // Look up the actor to get its tenant_id for the token claims
        let actor = ActorRepo::new(context)
            .get(*actor_id)
            .await?
            .ok_or_else(|| TicketError::ActorNotFound(*actor_id))?;

        let claims = TokenClaims::builder()
            .brain_id(brain.id)
            .tenant_id(actor.tenant_id)
            .actor_id(*actor_id)
            .build();

        let token = Token::issue(claims);
        // For non-distribution tickets, the target is the brain itself —
        // "this ticket grants access to this brain." Distribution tickets
        // minted by `bookmark share` will use `Ref::bookmark(id)` as the
        // target via a different code path (Act 3).
        let link = Link::new(Ref::brain(brain.id), token);
        let ticket = Ticket::builder()
            .actor_id(*actor_id)
            .brain_name(brain_name.clone())
            .brain_id(brain.id)
            .link(link)
            .granted_by(*actor_id)
            .build();

        context
            .emit(TicketEvents::TicketIssued(ticket.clone()))
            .await?;
        Ok(TicketResponse::Created(ticket))
    }

    pub async fn get(
        context: &SystemContext,
        selector: &GetTicket,
    ) -> Result<TicketResponse, TicketError> {
        let ticket = TicketRepo::new(context)
            .get(&selector.id)
            .await?
            .ok_or_else(|| TicketError::NotFound(selector.id))?;
        Ok(TicketResponse::Found(ticket))
    }

    pub async fn list(
        context: &SystemContext,
        ListTickets { filters }: &ListTickets,
    ) -> Result<TicketResponse, TicketError> {
        let listed = TicketRepo::new(context).list(filters).await?;
        Ok(TicketResponse::Listed(listed))
    }

    pub async fn validate(
        context: &SystemContext,
        ValidateTicket { token }: &ValidateTicket,
    ) -> Result<TicketResponse, TicketError> {
        let ticket = TicketRepo::new(context)
            .get_by_token(token.as_str())
            .await?
            .ok_or(TicketError::InvalidToken)?;
        Ok(TicketResponse::Validated(ticket))
    }
}
