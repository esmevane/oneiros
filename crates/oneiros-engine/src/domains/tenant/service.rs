use crate::*;

pub struct TenantService;

impl TenantService {
    pub async fn create(
        context: &HostLog,
        request: &CreateTenant,
    ) -> Result<TenantResponse, TenantError> {
        let CreateTenant::V1(create) = request;

        let tenant = Tenant::builder().name(create.name.clone()).build();

        context
            .emit(TenantEvents::TenantCreated(
                TenantCreated::builder_v1()
                    .tenant(tenant.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(TenantResponse::Created(
            TenantCreatedResponse::builder_v1()
                .tenant(tenant)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        context: &HostLog,
        request: &GetTenant,
    ) -> Result<TenantResponse, TenantError> {
        let GetTenant::V1(lookup) = request;
        let id = lookup.key.resolve()?;
        let scope = context.scope()?;
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
        context: &HostLog,
        request: &ListTenants,
    ) -> Result<TenantResponse, TenantError> {
        let ListTenants::V1(listing) = request;
        let scope = context.scope()?;
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
