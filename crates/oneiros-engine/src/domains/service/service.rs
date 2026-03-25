use std::ffi::OsString;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use service_manager::*;

use crate::*;

/// Service label constants — matches the legacy Context pattern.
const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "esmevane";
const APPLICATION: &str = "oneiros";

/// Health check backoff delays after starting the service.
const HEALTH_CHECK_DELAYS: &[Duration] = &[
    Duration::from_millis(200),
    Duration::from_millis(400),
    Duration::from_millis(800),
    Duration::from_millis(1600),
];

/// Helper to convert any error to ServiceError::Manager.
fn manager_err(e: impl std::fmt::Display) -> ServiceError {
    ServiceError::Manager(e.to_string())
}

pub struct ServiceService;

impl ServiceService {
    /// The service label for OS service manager registration.
    pub fn label() -> String {
        format!("{QUALIFIER}.{ORGANIZATION}.{APPLICATION}")
    }

    /// Get a native service manager set to user level.
    fn native_manager() -> Result<Box<dyn ServiceManager>, ServiceError> {
        let mut manager = <dyn ServiceManager>::native().map_err(manager_err)?;
        manager.set_level(ServiceLevel::User).map_err(manager_err)?;
        Ok(manager)
    }

    /// Parse the service label.
    fn parsed_label() -> Result<ServiceLabel, ServiceError> {
        Self::label().parse().map_err(manager_err)
    }

    /// Install the service as a managed user service.
    pub fn install(data_dir: &PathBuf) -> Result<ServiceResponse, ServiceError> {
        let label = Self::label();
        let manager = Self::native_manager()?;

        // Ensure log directory exists.
        let log_dir = data_dir.join("logs");
        std::fs::create_dir_all(&log_dir)?;

        let program = std::env::current_exe()?;

        manager
            .install(ServiceInstallCtx {
                label: Self::parsed_label()?,
                program,
                args: vec![OsString::from("service"), OsString::from("run")],
                contents: None,
                username: None,
                working_directory: Some(data_dir.clone()),
                environment: None,
                autostart: true,
                restart_policy: RestartPolicy::OnFailure {
                    delay_secs: Some(5),
                },
            })
            .map_err(manager_err)?;

        Ok(ServiceResponse::ServiceInstalled(label))
    }

    /// Uninstall the managed service. Best-effort stop before removal.
    pub fn uninstall() -> Result<ServiceResponse, ServiceError> {
        let manager = Self::native_manager()?;
        let label = Self::parsed_label()?;

        // Best-effort stop before uninstall.
        let _ = manager.stop(ServiceStopCtx {
            label: label.clone(),
        });

        manager
            .uninstall(ServiceUninstallCtx { label })
            .map_err(manager_err)?;

        Ok(ServiceResponse::ServiceUninstalled)
    }

    /// Start the managed service via OS service manager, then health check.
    pub async fn start(service_addr: SocketAddr) -> Result<ServiceResponse, ServiceError> {
        let manager = Self::native_manager()?;
        let label = Self::parsed_label()?;

        manager
            .start(ServiceStartCtx { label })
            .map_err(manager_err)?;

        // Health check with backoff.
        let base_url = format!("http://{service_addr}");
        let client = reqwest::Client::new();

        for delay in HEALTH_CHECK_DELAYS {
            tokio::time::sleep(*delay).await;

            if client
                .get(format!("{base_url}/health"))
                .send()
                .await
                .is_ok()
            {
                return Ok(ServiceResponse::ServiceHealthy(service_addr.to_string()));
            }
        }

        Ok(ServiceResponse::ServiceStarted)
    }

    /// Stop the managed service via OS service manager.
    pub fn stop() -> Result<ServiceResponse, ServiceError> {
        let manager = Self::native_manager()?;
        let label = Self::parsed_label()?;

        manager
            .stop(ServiceStopCtx { label })
            .map_err(manager_err)?;

        Ok(ServiceResponse::ServiceStopped)
    }

    /// Check if the service is running via health endpoint.
    pub async fn status(service_addr: SocketAddr) -> ServiceResponse {
        let base_url = format!("http://{service_addr}");
        let client = reqwest::Client::new();

        match client.get(format!("{base_url}/health")).send().await {
            Ok(resp) if resp.status().is_success() => {
                ServiceResponse::ServiceRunning(service_addr.to_string())
            }
            Ok(resp) => ServiceResponse::ServiceNotRunning(format!("HTTP {}", resp.status())),
            Err(e) => ServiceResponse::ServiceNotRunning(e.to_string()),
        }
    }
}
