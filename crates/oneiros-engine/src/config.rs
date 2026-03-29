use bon::Builder;
use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};

use crate::*;

/// Detect the default brain name from the current working directory.
fn detect_brain_name() -> BrainName {
    let cwd = std::env::current_dir().unwrap_or_default();

    ProjectDetector::default()
        .detect(&cwd)
        .map(|root| BrainName::new(root.name))
        .unwrap_or_default()
}

/// Resolve the default data directory from the platform.
fn default_data_dir() -> PathBuf {
    Platform::default().data_dir()
}

/// Configuration for the engine.
///
/// Carries paths, service address, and tuning knobs. Shared between
/// Server (which binds to the address) and Client (which connects to it).
#[derive(Builder, Debug, Clone, Parser)]
pub struct Config {
    /// Root directory for brain data (blobs, exports, etc.)
    #[arg(long, short, global = true, default_value_os_t = default_data_dir())]
    #[builder(default = default_data_dir())]
    pub data_dir: PathBuf,
    /// The brain (project) name. Auto-detected from cwd if not specified.
    #[arg(long, short, global = true, default_value_t = detect_brain_name())]
    #[builder(into, default = detect_brain_name())]
    pub brain: BrainName,
    /// Service management configuration.
    #[arg(skip = ServiceConfig::default())]
    #[builder(default)]
    pub service: ServiceConfig,
    /// Default dream assembly configuration.
    #[arg(skip = DreamConfig::default())]
    #[builder(default)]
    pub dream: DreamConfig,
    /// Output format: json (default), text, or prompt.
    #[arg(long, short, default_value = "json", global = true)]
    #[builder(default)]
    pub output: OutputMode,
}

impl Config {
    /// The service address (convenience accessor).
    pub fn service_addr(&self) -> SocketAddr {
        self.service.address
    }

    /// The base URL for HTTP clients to connect to the service.
    pub fn base_url(&self) -> String {
        format!("http://{}", self.service.address)
    }

    /// Path to the brain's data directory.
    pub fn brain_dir(&self) -> PathBuf {
        self.data_dir.join(self.brain.as_str())
    }

    /// Open the system database.
    pub fn system_db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        rusqlite::Connection::open(self.data_dir.join("system.db"))
    }

    /// Open the brain (project) database.
    pub fn brain_db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        rusqlite::Connection::open(self.brain_dir().join("brain.db"))
    }

    /// Path to the token file for the current brain.
    pub fn token_path(&self) -> PathBuf {
        self.data_dir
            .join("tickets")
            .join(format!("{}.token", self.brain))
    }

    /// Read the token for the current brain, if one exists.
    pub fn token(&self) -> Option<Token> {
        std::fs::read_to_string(self.token_path())
            .ok()
            .map(|s| Token::from(s.trim()))
    }

    /// Ensure the data directories and database schemas exist.
    ///
    /// Creates the data_dir, brain_dir, tickets dir, and runs
    /// EventLog + projection migrations on both system and brain databases.
    pub fn bootstrap(&self) -> Result<(), Error> {
        // Ensure directories
        std::fs::create_dir_all(&self.data_dir)
            .map_err(|e| Error::Context(format!("data_dir: {e}")))?;
        std::fs::create_dir_all(self.brain_dir())
            .map_err(|e| Error::Context(format!("brain_dir: {e}")))?;

        // System database
        let system_db = self
            .system_db()
            .map_err(|e| Error::Context(format!("system_db: {e}")))?;
        EventLog::new(&system_db).migrate()?;
        Projections::system().migrate(&system_db)?;

        // Brain database
        let brain_db = self
            .brain_db()
            .map_err(|e| Error::Context(format!("brain_db: {e}")))?;
        EventLog::new(&brain_db).migrate()?;
        Projections::project().migrate(&brain_db)?;

        Ok(())
    }

    /// Build a system context from this config.
    pub fn system(&self) -> SystemContext {
        SystemContext::new(self.clone())
    }

    /// Build a project context from this config.
    pub fn project(&self) -> ProjectContext {
        ProjectContext::new(self.clone())
    }
}
