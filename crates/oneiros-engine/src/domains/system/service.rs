use crate::*;

pub struct SystemService;

impl SystemService {
    pub async fn init(
        context: &SystemContext,
        request: &InitSystem,
    ) -> Result<SystemResponse, SystemError> {
        // system init is the bootstrapper — ensure data dir and schema exist.
        std::fs::create_dir_all(&context.config.data_dir)?;
        let db = context.db()?;
        EventLog::new(&db).migrate()?;
        context.projections.migrate(&db)?;
        drop(db);

        let tenants = TenantRepo::new(context).list().await?;

        if !tenants.is_empty() {
            return Ok(SystemResponse::HostAlreadyInitialized);
        }

        let name = request
            .name
            .clone()
            .unwrap_or_else(|| "oneiros user".to_string());

        let tenant_name = TenantName::new(&name);

        TenantService::create(
            context,
            &CreateTenant::builder().name(tenant_name.clone()).build(),
        )
        .await?;

        let tenants = TenantRepo::new(context).list().await?;

        if let Some(tenant) = tenants.first() {
            ActorService::create(
                context,
                &CreateActor::builder()
                    .tenant_id(tenant.id)
                    .name(ActorName::new(&name))
                    .build(),
            )
            .await?;
        }

        Ok(SystemResponse::SystemInitialized(tenant_name))
    }
}
