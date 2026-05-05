use crate::*;

pub struct SystemService;

impl SystemService {
    /// Initialize the host system. Bootstrapping order:
    ///
    /// 1. ensure data dir, host key, system db schema, projection migrations
    ///    — this is a precondition for any scope composition or actor work
    /// 2. compose `Scope<AtHost>` (now that data_dir exists)
    /// 3. idempotency check via `TenantRepo::list`
    /// 4. dispatch the bootstrap tenant via the bus
    /// 5. dispatch the bootstrap actor via the bus
    pub async fn init(
        config: &Config,
        mailbox: &Mailbox,
        request: &InitSystem,
    ) -> Result<SystemResponse, SystemError> {
        let details = request.current()?;

        std::fs::create_dir_all(&config.data_dir)?;
        HostKey::new(&config.data_dir).ensure()?;
        let host_db = HostDb::open_with(&config.platform()).await?;
        EventLog::new(&host_db).init()?;
        Projections::system().migrate(&host_db)?;
        drop(host_db);

        let scope = ComposeScope::new(config.clone()).host()?;

        let all_filters = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let tenants = TenantRepo::new(&scope).list(&all_filters).await?;

        if tenants.total > 0 {
            return Ok(SystemResponse::HostAlreadyInitialized);
        }

        let name = details
            .name
            .clone()
            .unwrap_or_else(|| "oneiros user".to_string());
        let tenant_name = TenantName::new(&name);

        let tenant_response = TenantService::create(
            &scope,
            mailbox,
            &CreateTenant::builder_v1()
                .name(tenant_name.clone())
                .build()
                .into(),
        )
        .await?;

        let tenant = match tenant_response {
            TenantResponse::Created(TenantCreatedResponse::V1(created)) => created.tenant,
            other => {
                return Err(SystemError::UnexpectedResponse(format!(
                    "expected Tenant(Created), got {other:?}"
                )));
            }
        };

        ActorService::create(
            &scope,
            mailbox,
            &CreateActor::builder_v1()
                .tenant_id(tenant.id)
                .name(ActorName::new(&name))
                .build()
                .into(),
        )
        .await?;

        Ok(SystemResponse::SystemInitialized(
            SystemInitializedResponse::builder_v1()
                .tenant(tenant_name)
                .build()
                .into(),
        ))
    }
}
