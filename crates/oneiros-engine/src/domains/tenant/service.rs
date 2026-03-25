use crate::*;

pub struct TenantService;

impl TenantService {
    pub async fn create(
        context: &SystemContext,
        name: TenantName,
    ) -> Result<TenantResponse, TenantError> {
        let tenant = Tenant::builder().name(name).build();

        context
            .emit(TenantEvents::TenantCreated(tenant.clone()))
            .await?;
        Ok(TenantResponse::Created(tenant))
    }

    pub async fn get(
        context: &SystemContext,
        id: &TenantId,
    ) -> Result<TenantResponse, TenantError> {
        let tenant = TenantRepo::new(context)
            .get(id)
            .await?
            .ok_or_else(|| TenantError::NotFound(*id))?;

        Ok(TenantResponse::Found(tenant))
    }

    pub async fn list(context: &SystemContext) -> Result<TenantResponse, TenantError> {
        let tenants = TenantRepo::new(context).list().await?;

        Ok(TenantResponse::Listed(tenants))
    }
}
