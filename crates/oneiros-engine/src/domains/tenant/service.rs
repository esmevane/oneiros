use chrono::Utc;

use crate::*;

pub struct TenantService;

impl TenantService {
    pub fn create(
        context: &SystemContext,
        name: TenantName,
    ) -> Result<TenantResponse, TenantError> {
        let tenant = Tenant {
            id: TenantId::new(),
            name,
            created_at: Utc::now().to_rfc3339(),
        };

        context.emit(TenantEvents::TenantCreated(tenant.clone()));
        Ok(TenantResponse::Created(tenant))
    }

    pub fn get(context: &SystemContext, id: &TenantId) -> Result<TenantResponse, TenantError> {
        let tenant = context
            .with_db(|conn| TenantRepo::new(conn).get(id))
            .map_err(TenantError::Database)?
            .ok_or_else(|| TenantError::NotFound(id.clone()))?;
        Ok(TenantResponse::Found(tenant))
    }

    pub fn list(context: &SystemContext) -> Result<TenantResponse, TenantError> {
        let tenants = context
            .with_db(|conn| TenantRepo::new(conn).list())
            .map_err(TenantError::Database)?;
        Ok(TenantResponse::Listed(tenants))
    }
}
