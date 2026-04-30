use crate::*;

pub struct SystemService;

impl SystemService {
    pub async fn init(
        context: &HostLog,
        request: &InitSystem,
    ) -> Result<SystemResponse, SystemError> {
        let details = request.current()?;
        // system init is the bootstrapper — ensure data dir, schema, and
        // host keypair all exist. The keypair establishes host identity
        // before the server ever starts.
        std::fs::create_dir_all(&context.config.data_dir)?;
        context.config.ensure_host_secret_key()?;
        let db = context.db()?;
        EventLog::new(&db).init()?;
        context.projections.migrate(&db)?;
        drop(db);

        let all_filters = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };

        let scope = context.scope()?;
        let tenants = TenantRepo::new(scope).list(&all_filters).await?;

        if tenants.total > 0 {
            return Ok(SystemResponse::HostAlreadyInitialized);
        }

        let name = details
            .name
            .clone()
            .unwrap_or_else(|| "oneiros user".to_string());

        let tenant_name = TenantName::new(&name);

        TenantService::create(
            context,
            &CreateTenant::builder_v1()
                .name(tenant_name.clone())
                .build()
                .into(),
        )
        .await?;

        let tenants = TenantRepo::new(scope).list(&all_filters).await?;

        if let Some(tenant) = tenants.items.first() {
            ActorService::create(
                context,
                &CreateActor::builder_v1()
                    .tenant_id(tenant.id)
                    .name(ActorName::new(&name))
                    .build()
                    .into(),
            )
            .await?;
        }

        Ok(SystemResponse::SystemInitialized(
            SystemInitializedResponse::builder_v1()
                .tenant(tenant_name)
                .build()
                .into(),
        ))
    }
}
