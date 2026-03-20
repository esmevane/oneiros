use crate::*;

pub struct SystemService;

impl SystemService {
    pub fn init(
        ctx: &SystemContext,
        name: String,
    ) -> Result<SystemResponse, Box<dyn std::error::Error>> {
        let tenants = ctx
            .with_db(|conn| TenantRepo::new(conn).list())
            .map_err(|e| format!("database error: {e}"))?;

        if !tenants.is_empty() {
            return Ok(SystemResponse::HostAlreadyInitialized);
        }

        let tenant_name = TenantName::new(&name);

        TenantService::create(ctx, TenantName::new(&name))?;

        let tenants = ctx
            .with_db(|conn| TenantRepo::new(conn).list())
            .map_err(|e| format!("database error: {e}"))?;

        if let Some(tenant) = tenants.first() {
            ActorService::create(ctx, tenant.id, ActorName::new(&name))?;
        }

        Ok(SystemResponse::SystemInitialized(tenant_name))
    }
}
