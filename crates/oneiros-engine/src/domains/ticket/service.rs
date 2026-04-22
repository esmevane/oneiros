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
        let brain = BrainRepo::new(context)
            .get(brain_name)
            .await?
            .ok_or_else(|| TicketError::BrainNotFound(brain_name.clone()))?;

        let target = Ref::brain(brain.id);
        let ticket = Self::issue(context, brain_name, &brain, *actor_id, target).await?;
        Ok(TicketResponse::Created(ticket))
    }

    /// Issue a ticket scoped to a specific target ref. Used by both
    /// `create` (brain-scoped) and `bookmark share` (bookmark-scoped).
    pub async fn issue(
        context: &SystemContext,
        brain_name: &BrainName,
        brain: &Brain,
        actor_id: ActorId,
        target: Ref,
    ) -> Result<Ticket, TicketError> {
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
        let link = Link::new(target, token);
        let ticket = Ticket::builder()
            .actor_id(actor_id)
            .brain_name(brain_name.clone())
            .brain_id(brain.id)
            .link(link)
            .granted_by(actor_id)
            .build();

        context
            .emit(TicketEvents::TicketIssued(ticket.clone()))
            .await?;

        Ok(ticket)
    }

    pub async fn get(
        context: &SystemContext,
        selector: &GetTicket,
    ) -> Result<TicketResponse, TicketError> {
        let id = selector.key.resolve()?;
        let ticket = TicketRepo::new(context)
            .get(&id)
            .await?
            .ok_or(TicketError::NotFound(id))?;
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
