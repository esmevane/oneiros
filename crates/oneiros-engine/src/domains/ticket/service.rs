use crate::*;

pub struct TicketService;

impl TicketService {
    pub async fn create(
        context: &SystemContext,
        request: &CreateTicket,
    ) -> Result<TicketResponse, TicketError> {
        let CreateTicket::V1(create) = request;
        let brain = BrainRepo::new(context)
            .get(&create.brain_name)
            .await?
            .ok_or_else(|| TicketError::BrainNotFound(create.brain_name.clone()))?;

        let target = Ref::brain(brain.id);
        let ticket =
            Self::issue(context, &create.brain_name, &brain, create.actor_id, target).await?;
        Ok(TicketResponse::Created(
            TicketCreatedResponse::builder_v1()
                .ticket(ticket)
                .build()
                .into(),
        ))
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
            .emit(TicketEvents::TicketIssued(
                TicketIssued::builder_v1()
                    .ticket(ticket.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(ticket)
    }

    pub async fn get(
        context: &SystemContext,
        request: &GetTicket,
    ) -> Result<TicketResponse, TicketError> {
        let GetTicket::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let ticket = TicketRepo::new(context)
            .get(&id)
            .await?
            .ok_or(TicketError::NotFound(id))?;
        Ok(TicketResponse::Found(
            TicketFoundResponse::builder_v1()
                .ticket(ticket)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        context: &SystemContext,
        request: &ListTickets,
    ) -> Result<TicketResponse, TicketError> {
        let ListTickets::V1(listing) = request;
        let listed = TicketRepo::new(context).list(&listing.filters).await?;
        Ok(TicketResponse::Listed(
            TicketsResponse::builder_v1()
                .items(listed.items)
                .total(listed.total)
                .build()
                .into(),
        ))
    }

    pub async fn validate(
        context: &SystemContext,
        request: &ValidateTicket,
    ) -> Result<TicketResponse, TicketError> {
        let ValidateTicket::V1(validate) = request;
        let ticket = TicketRepo::new(context)
            .get_by_token(validate.token.as_str())
            .await?
            .ok_or(TicketError::InvalidToken)?;
        Ok(TicketResponse::Validated(
            TicketValidatedResponse::builder_v1()
                .ticket(ticket)
                .build()
                .into(),
        ))
    }
}
