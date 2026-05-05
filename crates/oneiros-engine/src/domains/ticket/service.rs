use crate::*;

pub struct TicketService;

impl TicketService {
    pub async fn create(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &CreateTicket,
    ) -> Result<TicketResponse, TicketError> {
        let CreateTicket::V1(create) = request;
        let brain = BrainRepo::new(scope)
            .get(&create.brain_name)
            .await?
            .ok_or_else(|| TicketError::BrainNotFound(create.brain_name.clone()))?;

        let target = Ref::brain(brain.id);
        let ticket = Self::issue(
            scope,
            mailbox,
            &create.brain_name,
            &brain,
            create.actor_id,
            target,
        )
        .await?;
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
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        brain_name: &BrainName,
        brain: &Brain,
        actor_id: ActorId,
        target: Ref,
    ) -> Result<Ticket, TicketError> {
        let actor = ActorRepo::new(scope)
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
        let id = ticket.id;

        let new_event = NewEvent::builder()
            .data(Events::Ticket(TicketEvents::TicketIssued(
                TicketIssued::builder_v1().ticket(ticket).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let stored = TicketRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(TicketError::NotFound(id))?;

        Ok(stored)
    }

    pub async fn get(
        scope: &Scope<AtHost>,
        request: &GetTicket,
    ) -> Result<TicketResponse, TicketError> {
        let GetTicket::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let ticket = TicketRepo::new(scope)
            .fetch(&id)
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
        scope: &Scope<AtHost>,
        request: &ListTickets,
    ) -> Result<TicketResponse, TicketError> {
        let ListTickets::V1(listing) = request;
        let listed = TicketRepo::new(scope).list(&listing.filters).await?;
        Ok(TicketResponse::Listed(
            TicketsResponse::builder_v1()
                .items(listed.items)
                .total(listed.total)
                .build()
                .into(),
        ))
    }

    pub async fn validate(
        scope: &Scope<AtHost>,
        request: &ValidateTicket,
    ) -> Result<TicketResponse, TicketError> {
        let ValidateTicket::V1(validate) = request;
        let ticket = TicketRepo::new(scope)
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
