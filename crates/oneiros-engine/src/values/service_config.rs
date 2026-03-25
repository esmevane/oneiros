use bon::Builder;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;

use crate::Platform;

const DEFAULT_DELAYS_MS: [u64; 4] = [200, 400, 800, 1600];

/// Configuration for the managed service.
///
/// Carries the service identity, health check behavior, and network
/// address. Lives inside `Config` as the `[service]` section.
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// The service label for OS registration (e.g., "com.esmevane.oneiros").
    #[builder(default = Platform::default().service_label())]
    pub label: String,
    /// Address the service listens on.
    #[builder(default = "127.0.0.1:2100".parse::<SocketAddr>().unwrap())]
    pub address: SocketAddr,
    /// Health check retry delays after starting (milliseconds).
    #[builder(default = DEFAULT_DELAYS_MS.to_vec())]
    pub health_check_delays_ms: Vec<u64>,
    /// Restart delay on failure (seconds).
    #[builder(default = 5)]
    pub restart_delay_secs: u32,
}

impl ServiceConfig {
    /// Health check delays as `Duration`s.
    pub fn health_check_delays(&self) -> Vec<Duration> {
        self.health_check_delays_ms
            .iter()
            .map(|&ms| Duration::from_millis(ms))
            .collect()
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
