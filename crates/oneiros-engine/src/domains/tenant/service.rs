use crate::*;

pub(crate) struct TenantService;

impl TenantService {
    /// Create a tenant by telling the bus about `TenantCreated` and waiting
    /// until the event is persisted in the log.
    ///
    /// The response carries the record from the event itself — the same
    /// payload the projection applies. Once the event is persisted the
    /// request succeeds; reading the projection back here could fail a
    /// write that already happened, and the client's retry would
    /// duplicate it.
    pub(crate) async fn create(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &CreateTenant,
    ) -> Result<TenantResponse, TenantError> {
        let CreateTenant::V1(create) = request;

        let tenant = Tenant::builder().name(create.name.clone()).build();

        let new_event = NewEvent::builder()
            .data(Events::Tenant(TenantEvents::TenantCreated(
                TenantCreated::builder_v1()
                    .tenant(tenant.clone())
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(scope.append(new_event.clone()));
        scope.ensure_persisted(&new_event).await?;

        Ok(TenantResponse::Created(
            TenantCreatedResponse::builder_v1()
                .tenant(tenant)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn get(
        scope: &Scope<AtHost>,
        request: &GetTenant,
    ) -> Result<TenantResponse, TenantError> {
        let GetTenant::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let tenant = TenantRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(TenantError::NotFound(id))?;
        Ok(TenantResponse::Found(
            TenantFoundResponse::builder_v1()
                .tenant(tenant)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn list(
        scope: &Scope<AtHost>,
        request: &ListTenants,
    ) -> Result<TenantResponse, TenantError> {
        let ListTenants::V1(listing) = request;
        let listed = TenantRepo::new(scope).list(&listing.filters).await?;
        Ok(TenantResponse::Listed(
            TenantsResponse::builder_v1()
                .items(listed.items)
                .total(listed.total)
                .build()
                .into(),
        ))
    }
}
