use chrono::Utc;
use uuid::Uuid;

use crate::contexts::SystemContext;

use super::errors::TenantError;
use super::events::*;
use super::model::Tenant;
use super::repo::TenantRepo;
use super::responses::TenantResponse;

pub struct TenantService;

impl TenantService {
    pub fn create(ctx: &SystemContext, name: String) -> Result<TenantResponse, TenantError> {
        let tenant = Tenant {
            id: Uuid::now_v7().to_string(),
            name,
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit(TenantEvents::TenantCreated(tenant.clone()));
        Ok(TenantResponse::Created(tenant))
    }

    pub fn get(ctx: &SystemContext, id: &str) -> Result<TenantResponse, TenantError> {
        let tenant = ctx
            .with_db(|conn| TenantRepo::new(conn).get(id))
            .map_err(TenantError::Database)?
            .ok_or_else(|| TenantError::NotFound(id.to_string()))?;
        Ok(TenantResponse::Found(tenant))
    }

    pub fn list(ctx: &SystemContext) -> Result<TenantResponse, TenantError> {
        let tenants = ctx
            .with_db(|conn| TenantRepo::new(conn).list())
            .map_err(TenantError::Database)?;
        Ok(TenantResponse::Listed(tenants))
    }
}
