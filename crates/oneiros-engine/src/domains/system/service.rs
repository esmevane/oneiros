use crate::*;

pub(crate) struct SystemService;

impl SystemService {
    pub(crate) async fn init(
        context: &SystemContext,
        request: &InitSystem,
    ) -> Result<SystemResponse, SystemError> {
        // system init is the bootstrapper — ensure data dir and schema exist.
        std::fs::create_dir_all(&context.config.data_dir)?;
        let db = context.db()?;
        EventLog::new(&db).migrate()?;
        context.projections.migrate(&db)?;
        drop(db);

        let all_filters = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };

        let tenants = TenantRepo::new(context).list(&all_filters).await?;

        if tenants.total > 0 {
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

        let tenants = TenantRepo::new(context).list(&all_filters).await?;

        if let Some(tenant) = tenants.items.first() {
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
