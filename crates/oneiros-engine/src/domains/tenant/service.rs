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

    pub fn get(context: &SystemContext, id: &TenantId) -> Result<TenantResponse, TenantError> {
        let tenant = TenantRepo::new(&context.db()?)
            .get(id)?
            .ok_or_else(|| TenantError::NotFound(*id))?;

        Ok(TenantResponse::Found(tenant))
    }

    pub fn list(context: &SystemContext) -> Result<TenantResponse, TenantError> {
        let tenants = TenantRepo::new(&context.db()?).list()?;

        Ok(TenantResponse::Listed(tenants))
    }
}
