use bon::Builder;
use clap::Args;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;

use crate::*;

const DEFAULT_DELAYS_MS: [u64; 4] = [200, 400, 800, 1600];

fn default_label() -> String {
    Platform::default().service_label()
}

fn default_address() -> SocketAddr {
    "127.0.0.1:2100".parse().unwrap()
}

fn default_delays() -> Vec<u64> {
    DEFAULT_DELAYS_MS.to_vec()
}

fn default_restart_delay() -> u32 {
    5
}

/// Configuration for the managed service.
///
/// Carries the service identity, health check behavior, and network
/// address. Lives inside `Config` as the `[service]` section.
#[derive(Args, Builder, Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceConfig {
    /// The service label for OS registration (e.g., "com.esmevane.oneiros").
    #[arg(long, global = true, default_value_t = default_label())]
    #[builder(default = default_label())]
    #[serde(default = "default_label")]
    pub label: String,
    /// Address the service listens on.
    #[arg(long, global = true, default_value_t = default_address())]
    #[builder(default = default_address())]
    #[serde(default = "default_address")]
    pub address: SocketAddr,
    /// Health check retry delays after starting (milliseconds).
    #[arg(skip = default_delays())]
    #[builder(default = default_delays())]
    #[serde(default = "default_delays")]
    pub health_check_delays_ms: Vec<u64>,
    /// Restart delay on failure (seconds).
    #[arg(long, global = true, default_value_t = 5)]
    #[builder(default = 5)]
    #[serde(default = "default_restart_delay")]
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
