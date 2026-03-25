//! Engine configuration — pure data, loaded at startup.
use std::{net::SocketAddr, path::PathBuf};

use crate::{DreamConfig, ServiceConfig};

/// Configuration for the engine.
///
/// Carries paths, service address, and tuning knobs. Shared between
/// Server (which binds to the address) and Client (which connects to it).
#[derive(Debug, Clone)]
pub struct Config {
    /// Root directory for brain data (blobs, exports, etc.)
    pub data_dir: PathBuf,
    /// Service management configuration.
    pub service: ServiceConfig,
    /// Default dream assembly configuration.
    pub dream: DreamConfig,
}

impl Config {
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
            service: ServiceConfig::default(),
            dream: DreamConfig::default(),
        }
    }

    /// The service address (convenience accessor).
    pub fn service_addr(&self) -> SocketAddr {
        self.service.addr
    }

    /// The base URL for HTTP clients to connect to the service.
    pub fn base_url(&self) -> String {
        format!("http://{}", self.service.addr)
    }

    /// Builder-style setter for the service address.
    pub fn with_service_addr(mut self, addr: SocketAddr) -> Self {
        self.service.addr = addr;
        self
    }
}
