use crate::*;

pub struct TenantService;

impl TenantService {
    /// Create a tenant by dispatching `TenantCreated` through the bus
    /// and reading the eventually-consistent record back via fetch.
    ///
    /// No phantom state — the response carries whatever the projection
    /// has seen, never a synthesized record. If the fetch window
    /// expires before the projection catches up, this surfaces as
    /// `TenantError::NotFound`.
    pub async fn create(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        request: &CreateTenant,
    ) -> Result<TenantResponse, TenantError> {
        let CreateTenant::V1(create) = request;

        let tenant = Tenant::builder().name(create.name.clone()).build();
        let id = tenant.id;

        let new_event = NewEvent::builder()
            .data(Events::Tenant(TenantEvents::TenantCreated(
                TenantCreated::builder_v1().tenant(tenant).build().into(),
            )))
            .build();

        mailbox.tell(Message::new(scope.clone(), new_event));

        let stored = TenantRepo::new(scope)
            .fetch(&id)
            .await?
            .ok_or(TenantError::NotFound(id))?;

        Ok(TenantResponse::Created(
            TenantCreatedResponse::builder_v1()
                .tenant(stored)
                .build()
                .into(),
        ))
    }

    pub async fn get(
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

    pub async fn list(
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
