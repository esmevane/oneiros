use crate::*;

pub(crate) struct TicketService;

impl TicketService {
    pub(crate) async fn create(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &CreateTicket,
    ) -> Result<TicketResponse, TicketError> {
        let CreateTicket::V1(create) = request;
        let project = ProjectRepo::new(scope)
            .get(&create.project_name)
            .await?
            .ok_or_else(|| TicketError::ProjectNotFound(create.project_name.clone()))?;

        let target = Ref::project(project.id);
        let permissions: Vec<Permission> = create
            .permissions
            .iter()
            .map(|op| Permission::from(PermissionV1 { operation: *op }))
            .collect();
        let ticket = Self::issue(
            scope,
            mailbox,
            &create.project_name,
            &project,
            create.actor_id,
            target,
            permissions,
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
    /// `create` (project-scoped) and `bookmark share` (bookmark-scoped).
    pub(crate) async fn issue(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        project_name: &ProjectName,
        project: &Project,
        actor_id: ActorId,
        target: Ref,
        permissions: Vec<Permission>,
    ) -> Result<Ticket, TicketError> {
        let actor = ActorRepo::new(scope)
            .get(actor_id)
            .await?
            .ok_or_else(|| TicketError::ActorNotFound(actor_id))?;

        let claims = TokenClaims::builder()
            .project_id(project.id)
            .tenant_id(actor.tenant_id)
            .actor_id(actor_id)
            .build();

        let token = Token::issue(claims);
        let link = Link::new(target, token);

        let ticket = Ticket::builder()
            .actor_id(actor_id)
            .project_name(project_name.clone())
            .project_id(project.id)
            .link(link)
            .granted_by(actor_id)
            .permissions(permissions)
            .build();
        let id = ticket.id;

        let new_event = NewEvent::builder()
            .data(Events::Ticket(TicketEvents::TicketIssued(
                TicketIssued::builder_v1().ticket(ticket).build().into(),
            )))
            .build();

        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        let stored = TicketRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(TicketError::NotFound(id))?;

        Ok(stored)
    }

    pub(crate) async fn get(
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

    pub(crate) async fn list(
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

    pub(crate) async fn validate(
        scope: &Scope<AtHost>,
        request: &ValidateTicket,
    ) -> Result<TicketResponse, TicketError> {
        let ValidateTicket::V1(validate) = request;
        let ticket = TicketRepo::new(scope)
            .get_by_token(validate.token.as_str())
            .await?
            .ok_or(TicketError::InvalidToken)?;

        ticket
            .check_validity()
            .map_err(|_| TicketError::InvalidToken)?;

        Ok(TicketResponse::Validated(
            TicketValidatedResponse::builder_v1()
                .ticket(ticket)
                .build()
                .into(),
        ))
    }
}
