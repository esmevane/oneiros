use std::net::SocketAddr;
use std::time::Duration;

/// Configuration for the managed service.
///
/// Carries the service identity, health check behavior, and network
/// address. Lives inside `Config` as the `[service]` section.
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    /// The service label for OS registration (e.g., "com.esmevane.oneiros").
    pub label: String,
    /// Address the service listens on.
    pub addr: SocketAddr,
    /// Health check retry delays after starting.
    pub health_check_delays: Vec<Duration>,
    /// Restart delay on failure (seconds).
    pub restart_delay_secs: u32,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            label: "com.esmevane.oneiros".to_string(),
            addr: SocketAddr::from(([127, 0, 0, 1], 2100)),
            health_check_delays: vec![
                Duration::from_millis(200),
                Duration::from_millis(400),
                Duration::from_millis(800),
                Duration::from_millis(1600),
            ],
            restart_delay_secs: 5,
        }
    }
}
