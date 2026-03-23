//! Engine configuration — pure data, loaded at startup.

use std::net::SocketAddr;
use std::path::PathBuf;

use crate::DreamConfig;

/// Configuration for the engine.
///
/// Carries paths, service address, and tuning knobs. Shared between
/// Server (which binds to the address) and Client (which connects to it).
#[derive(Debug, Clone)]
pub struct Config {
    /// Root directory for brain data (blobs, exports, etc.)
    pub data_dir: PathBuf,
    /// Address the service listens on / clients connect to.
    pub service_addr: SocketAddr,
    /// Default dream assembly configuration.
    pub dream: DreamConfig,
}

impl Config {
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
            service_addr: default_addr(),
            dream: DreamConfig::default(),
        }
    }

    /// The base URL for HTTP clients to connect to the service.
    pub fn base_url(&self) -> String {
        format!("http://{}", self.service_addr)
    }

    /// Builder-style setter for the service address.
    pub fn with_service_addr(mut self, addr: SocketAddr) -> Self {
        self.service_addr = addr;
        self
    }
}

fn default_addr() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 2100))
}
