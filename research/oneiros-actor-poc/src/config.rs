//! Config actor — a resource without transport features.
//!
//! Config is an actor that holds configuration state and responds to
//! queries from other actors. It has no HTTP endpoints, no MCP tools,
//! no CLI commands. It's purely internal — queryable by other actors
//! through the registry.

use oneiros_actor::Actor;

/// Configuration values this actor manages.
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub data_dir: std::path::PathBuf,
    pub port: u16,
    pub grace_period_secs: u64,
}

/// Messages the config actor handles.
#[derive(Debug)]
pub enum ConfigMessage {
    GetDataDir,
    GetPort,
    GetAll,
}

/// Responses from the config actor.
#[derive(Debug, Clone)]
pub enum ConfigResponse {
    DataDir(std::path::PathBuf),
    Port(u16),
    All(ServiceConfig),
}

/// The config actor — holds config, responds to queries.
pub struct ConfigActor {
    config: ServiceConfig,
}

impl ConfigActor {
    pub fn new(config: ServiceConfig) -> Self {
        Self { config }
    }
}

impl Actor for ConfigActor {
    type Message = ConfigMessage;
    type Response = ConfigResponse;

    async fn handle(&mut self, message: ConfigMessage) -> ConfigResponse {
        match message {
            ConfigMessage::GetDataDir => ConfigResponse::DataDir(self.config.data_dir.clone()),
            ConfigMessage::GetPort => ConfigResponse::Port(self.config.port),
            ConfigMessage::GetAll => ConfigResponse::All(self.config.clone()),
        }
    }
}
