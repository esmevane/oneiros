use std::ffi::OsString;

use service_manager::*;

use crate::*;

/// Helper to convert any error to HostError::Manager.
fn manager_err(e: impl std::fmt::Display) -> HostError {
    HostError::Manager(e.to_string())
}

pub(crate) struct HostService;

impl HostService {
    /// Initialize the host system. Bootstrapping order:
    ///
    /// 1. ensure data dir, host key, host db schema, projection migrations
    ///    — this is a precondition for any scope composition or actor work
    /// 2. compose `Scope<AtHost>` (now that data_dir exists)
    /// 3. idempotency check via `TenantRepo::list`
    /// 4. dispatch the bootstrap tenant via the bus
    /// 5. dispatch the bootstrap actor via the bus
    pub(crate) async fn init(
        config: &Config,
        mailbox: &Mailbox,
        request: &InitHost,
    ) -> Result<HostResponse, HostError> {
        let details = request.current()?;

        config.platform().ensure_dir(&config.data_dir)?;
        HostKey::new(config.platform()).ensure()?;
        let host_db = HostDb::open_with(&config.platform()).await?;
        EventLog::new(&host_db).init()?;
        Projections::host().migrate(&host_db)?;
        drop(host_db);

        let scope = ComposeScope::new(config.clone()).host()?;

        let all_filters = SearchFilters {
            limit: Limit(usize::MAX),
            offset: Offset(0),
        };
        let tenants = TenantRepo::new(&scope).list(&all_filters).await?;

        if tenants.total > 0 {
            return Ok(HostResponse::HostAlreadyInitialized);
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
                return Err(HostError::UnexpectedResponse(format!(
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

        Ok(HostResponse::HostInitialized(
            HostInitializedResponse::builder_v1()
                .tenant(tenant_name)
                .build()
                .into(),
        ))
    }

    /// Acquire a native OS service manager set to user level.
    fn manager() -> Result<Box<dyn ServiceManager>, HostError> {
        let mut manager = <dyn ServiceManager>::native().map_err(manager_err)?;
        manager.set_level(ServiceLevel::User).map_err(manager_err)?;
        Ok(manager)
    }

    /// Parse a service label string.
    fn parse_label(label: &str) -> Result<ServiceLabel, HostError> {
        label.parse().map_err(manager_err)
    }

    /// Install oneiros as a managed user service.
    pub(crate) fn install(config: &Config) -> Result<HostResponse, HostError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.service.label)?;

        config.platform().ensure_dir(config.data_dir.join("logs"))?;

        manager
            .install(ServiceInstallCtx {
                label,
                program: std::env::current_exe()?,
                args: vec![OsString::from("host"), OsString::from("run")],
                contents: None,
                username: None,
                working_directory: Some(config.data_dir.to_path_buf()),
                environment: None,
                autostart: true,
                restart_policy: RestartPolicy::OnFailure {
                    delay_secs: Some(config.service.restart_delay_secs),
                    max_retries: None,
                    reset_after_secs: None,
                },
            })
            .map_err(manager_err)?;

        Ok(HostResponse::ServiceInstalled(
            ServiceInstalledResponse::builder_v1()
                .name(config.service.label.clone())
                .build()
                .into(),
        ))
    }

    /// Uninstall the managed service. Best-effort stop before removal.
    pub(crate) fn uninstall(config: &Config) -> Result<HostResponse, HostError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.service.label)?;

        let _ = manager.stop(ServiceStopCtx {
            label: label.clone(),
        });

        manager
            .uninstall(ServiceUninstallCtx { label })
            .map_err(manager_err)?;

        Ok(HostResponse::ServiceUninstalled)
    }

    /// Start the managed service, then health check with backoff.
    pub(crate) async fn start(config: &Config) -> Result<HostResponse, HostError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.service.label)?;

        manager
            .start(ServiceStartCtx { label })
            .map_err(manager_err)?;

        let client = reqwest::Client::new();
        let health_url = format!("{}/health", config.base_url());

        for delay in &config.service.health_check_delays() {
            tokio::time::sleep(*delay).await;

            if client.get(&health_url).send().await.is_ok() {
                return Ok(HostResponse::ServiceHealthy(
                    ServiceHealthyResponse::builder_v1()
                        .address(config.service.address.to_string())
                        .build()
                        .into(),
                ));
            }
        }

        Ok(HostResponse::ServiceStarted)
    }

    /// Stop the managed service.
    pub(crate) fn stop(config: &Config) -> Result<HostResponse, HostError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.service.label)?;

        manager
            .stop(ServiceStopCtx { label })
            .map_err(manager_err)?;

        Ok(HostResponse::ServiceStopped)
    }

    /// Check if the service is running via the health endpoint.
    pub(crate) async fn status(config: &Config) -> HostResponse {
        let client = reqwest::Client::new();
        let health_url = format!("{}/health", config.base_url());

        match client.get(&health_url).send().await {
            Ok(resp) if resp.status().is_success() => HostResponse::ServiceRunning(
                ServiceRunningResponse::builder_v1()
                    .address(config.service.address.to_string())
                    .build()
                    .into(),
            ),
            Ok(resp) => HostResponse::ServiceNotRunning(
                ServiceNotRunningResponse::builder_v1()
                    .reason(format!("HTTP {}", resp.status()))
                    .build()
                    .into(),
            ),
            Err(e) => HostResponse::ServiceNotRunning(
                ServiceNotRunningResponse::builder_v1()
                    .reason(e.to_string())
                    .build()
                    .into(),
            ),
        }
    }

    /// Run the HTTP server directly in the foreground.
    pub(crate) async fn run(config: &Config) -> Result<HostResponse, HostError> {
        Server::new(config.clone()).serve().await?;

        Ok(HostResponse::ServiceStopped)
    }
}
