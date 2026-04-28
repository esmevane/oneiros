use std::ffi::OsString;

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
    pub fn install(config: &Config) -> Result<ServiceResponse, ServiceError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.service.label)?;

        // Ensure log directory exists.
        std::fs::create_dir_all(config.data_dir.join("logs"))?;

        manager
            .install(ServiceInstallCtx {
                label,
                program: std::env::current_exe()?,
                args: vec![OsString::from("service"), OsString::from("run")],
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

        Ok(ServiceResponse::ServiceInstalled(ServiceName::new(
            config.service.label.clone(),
        )))
    }

    /// Uninstall the managed service. Best-effort stop before removal.
    pub fn uninstall(config: &Config) -> Result<ServiceResponse, ServiceError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.service.label)?;

        let _ = manager.stop(ServiceStopCtx {
            label: label.clone(),
        });

        manager
            .uninstall(ServiceUninstallCtx { label })
            .map_err(manager_err)?;

        Ok(ServiceResponse::ServiceUninstalled)
    }

    /// Start the managed service, then health check with backoff.
    pub async fn start(config: &Config) -> Result<ServiceResponse, ServiceError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.service.label)?;

        manager
            .start(ServiceStartCtx { label })
            .map_err(manager_err)?;

        let client = reqwest::Client::new();
        let health_url = format!("http://{}/health", config.service.address);

        for delay in &config.service.health_check_delays() {
            tokio::time::sleep(*delay).await;

            if client.get(&health_url).send().await.is_ok() {
                return Ok(ServiceResponse::ServiceHealthy(ServiceAddress::new(
                    config.service.address.to_string(),
                )));
            }
        }

        Ok(ServiceResponse::ServiceStarted)
    }

    /// Stop the managed service.
    pub fn stop(config: &Config) -> Result<ServiceResponse, ServiceError> {
        let manager = Self::manager()?;
        let label = Self::parse_label(&config.service.label)?;

        manager
            .stop(ServiceStopCtx { label })
            .map_err(manager_err)?;

        Ok(ServiceResponse::ServiceStopped)
    }

    /// Check if the service is running via health endpoint.
    pub async fn status(config: &Config) -> ServiceResponse {
        let client = reqwest::Client::new();
        let health_url = format!("http://{}/health", config.service.address);

        match client.get(&health_url).send().await {
            Ok(resp) if resp.status().is_success() => ServiceResponse::ServiceRunning(
                ServiceAddress::new(config.service.address.to_string()),
            ),
            Ok(resp) => ServiceResponse::ServiceNotRunning(ServiceReason::new(format!(
                "HTTP {}",
                resp.status()
            ))),
            Err(e) => ServiceResponse::ServiceNotRunning(ServiceReason::new(e.to_string())),
        }
    }

    /// Run the HTTP server directly in the foreground.
    pub async fn run(config: &Config) -> Result<ServiceResponse, ServiceError> {
        Server::new(config.clone()).start().await?;

        Ok(ServiceResponse::ServiceStopped)
    }
}
