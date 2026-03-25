use std::ffi::OsString;
use std::path::Path;

use service_manager::*;

use crate::*;

/// Helper to convert any error to ServiceError::Manager.
fn manager_err(e: impl std::fmt::Display) -> ServiceError {
    ServiceError::Manager(e.to_string())
}

/// Manages the OS-level service lifecycle.
///
/// Each method takes the configuration it needs — no internal state.
/// The OS service manager is acquired per-operation because it's not
/// guaranteed to be reusable across calls.
pub struct ServiceService;

impl ServiceService {
    /// Acquire a native service manager set to user level.
    fn manager() -> Result<Box<dyn ServiceManager>, ServiceError> {
        let mut manager = <dyn ServiceManager>::native().map_err(manager_err)?;
        manager.set_level(ServiceLevel::User).map_err(manager_err)?;
        Ok(manager)
    }

    /// Parse a service label string.
    fn parse_label(label: &str) -> Result<ServiceLabel, ServiceError> {
        label.parse().map_err(manager_err)
    }

    /// Install the service as a managed user service.
    pub fn install(
        config: &ServiceConfig,
        data_dir: &Path,
    ) -> Result<ServiceResponse, ServiceError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.label)?;

        // Ensure log directory exists.
        std::fs::create_dir_all(data_dir.join("logs"))?;

        manager
            .install(ServiceInstallCtx {
                label,
                program: std::env::current_exe()?,
                args: vec![OsString::from("service"), OsString::from("run")],
                contents: None,
                username: None,
                working_directory: Some(data_dir.to_path_buf()),
                environment: None,
                autostart: true,
                restart_policy: RestartPolicy::OnFailure {
                    delay_secs: Some(config.restart_delay_secs),
                },
            })
            .map_err(manager_err)?;

        Ok(ServiceResponse::ServiceInstalled(config.label.clone()))
    }

    /// Uninstall the managed service. Best-effort stop before removal.
    pub fn uninstall(config: &ServiceConfig) -> Result<ServiceResponse, ServiceError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.label)?;

        let _ = manager.stop(ServiceStopCtx {
            label: label.clone(),
        });

        manager
            .uninstall(ServiceUninstallCtx { label })
            .map_err(manager_err)?;

        Ok(ServiceResponse::ServiceUninstalled)
    }

    /// Start the managed service, then health check with backoff.
    pub async fn start(config: &ServiceConfig) -> Result<ServiceResponse, ServiceError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.label)?;

        manager
            .start(ServiceStartCtx { label })
            .map_err(manager_err)?;

        let client = reqwest::Client::new();
        let health_url = format!("http://{}/health", config.addr);

        for delay in &config.health_check_delays {
            tokio::time::sleep(*delay).await;

            if client.get(&health_url).send().await.is_ok() {
                return Ok(ServiceResponse::ServiceHealthy(config.addr.to_string()));
            }
        }

        Ok(ServiceResponse::ServiceStarted)
    }

    /// Stop the managed service.
    pub fn stop(config: &ServiceConfig) -> Result<ServiceResponse, ServiceError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.label)?;

        manager
            .stop(ServiceStopCtx { label })
            .map_err(manager_err)?;

        Ok(ServiceResponse::ServiceStopped)
    }

    /// Check if the service is running via health endpoint.
    pub async fn status(config: &ServiceConfig) -> ServiceResponse {
        let client = reqwest::Client::new();
        let health_url = format!("http://{}/health", config.addr);

        match client.get(&health_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                ServiceResponse::ServiceRunning(config.addr.to_string())
            }
            Ok(resp) => ServiceResponse::ServiceNotRunning(format!("HTTP {}", resp.status())),
            Err(e) => ServiceResponse::ServiceNotRunning(e.to_string()),
        }
    }

    /// Run the HTTP server directly in the foreground.
    pub async fn run(engine: &Engine) -> Result<ServiceResponse, ServiceError> {
        let router = engine
            .project_router()
            .map_err(|e| ServiceError::Manager(e.to_string()))?;

        let addr = engine.service_addr();
        let listener = tokio::net::TcpListener::bind(addr).await?;

        axum::serve(listener, router.into_make_service())
            .await
            .map_err(ServiceError::Io)?;

        Ok(ServiceResponse::ServiceStopped)
    }
}
