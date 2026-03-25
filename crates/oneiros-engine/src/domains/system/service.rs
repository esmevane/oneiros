use crate::*;

pub struct SystemService;

impl SystemService {
    pub async fn init(
        context: &SystemContext,
        name: String,
    ) -> Result<SystemResponse, SystemError> {
        let tenants = TenantRepo::new(context).list().await?;

        if !tenants.is_empty() {
            return Ok(SystemResponse::HostAlreadyInitialized);
        }

        let tenant_name = TenantName::new(&name);

        TenantService::create(context, TenantName::new(&name)).await?;

        let tenants = TenantRepo::new(context).list().await?;

        if let Some(tenant) = tenants.first() {
            ActorService::create(context, tenant.id, ActorName::new(&name)).await?;
        }

        Ok(SystemResponse::SystemInitialized(tenant_name))
    }
}
