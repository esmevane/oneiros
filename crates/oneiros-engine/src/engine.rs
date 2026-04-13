//! Engine — the formal consumer surface for oneiros.
//!
//! Every consumer of the engine (binary, tests, xtask, programmatic)
//! goes through this type. It owns a `Config` and provides entry points
//! for each use case:
//!
//! - `from_cli()` — parse CLI args, merge config file, set up tracing
//! - `new(config)` — explicit config (tests, programmatic use)
//! - `execute(cli)` — run a parsed CLI command
//! - `start()` — bind and serve HTTP/MCP
//! - `package(target)` — emit skill assets to a directory

use std::net::SocketAddr;
use std::path::Path;

use clap::Parser;
use tokio::net::TcpListener;

use crate::*;

/// The engine — entry point for all consumers.
pub(crate) struct Engine {
    config: Config,
}

impl Engine {
    /// From CLI args — parses arguments and merges config file.
    ///
    /// Returns the engine and the parsed CLI so the caller can execute
    /// and render. Tracing setup is the caller's responsibility.
    pub fn from_cli() -> Result<(Self, Cli), Error> {
        let cli = Cli::parse();
        let config = cli.config().clone().with_config_file();

        Ok((Self::new(config), cli))
    }

    /// From explicit config — tests and programmatic consumers.
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    /// Execute a parsed CLI command against this engine's config.
    pub async fn execute(&self, cli: &Cli) -> Result<Rendered<Responses>, Error> {
        cli.execute(&self.config).await
    }

    /// Start the HTTP server on the configured address.
    ///
    /// Binds to the address in config (use `127.0.0.1:0` for ephemeral ports),
    /// updates the config with the resolved address, and returns a handle.
    /// The server runs in a background task and stops when the handle is dropped.
    pub(crate) async fn start(&mut self) -> Result<ServerHandle, ServerError> {
        let listener = TcpListener::bind(self.config.service.address).await?;
        let address = listener.local_addr()?;

        // Update config so subsequent operations (CLI commands, clients)
        // connect to the actual bound address.
        self.config.service.address = address;

        let server = Server::new(self.config.clone());
        let handle = tokio::spawn(async move {
            server.serve(listener).await.expect("server failed");
        });

        Ok(ServerHandle { address, handle })
    }

    /// Emit skill package to a target directory.
    ///
    /// Static — no config or server needed. Writes all skill assets
    /// (SKILL.md, plugin.json, hooks, agents, commands, resources)
    /// to the target path.
    pub(crate) fn package(target: &Path) -> Result<usize, std::io::Error> {
        SkillPackage::install(target)
    }

    /// The resolved configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// A handle to a running server.
///
/// Holds the resolved address and a task handle. The server stops
/// when the handle is dropped.
pub(crate) struct ServerHandle {
    address: SocketAddr,
    handle: tokio::task::JoinHandle<()>,
}

impl ServerHandle {
    /// The address the server is actually listening on.
    ///
    /// When configured with port 0, this returns the OS-assigned port.
    pub(crate) fn address(&self) -> SocketAddr {
        self.address
    }
}

impl Drop for ServerHandle {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_emits_assets() {
        let temp = tempfile::TempDir::new().unwrap();
        let count = Engine::package(temp.path()).unwrap();
        assert!(count > 0, "should emit at least one file");
        assert!(temp.path().join("skills/oneiros/SKILL.md").exists());
        assert!(temp.path().join("commands/dream.md").exists());
    }

    #[tokio::test]
    async fn start_binds_ephemeral_port() {
        let dir = tempfile::TempDir::new().unwrap();
        let config = Config::builder()
            .data_dir(dir.path().to_path_buf())
            .brain(BrainName::new("test"))
            .service(
                ServiceConfig::builder()
                    .address("127.0.0.1:0".parse().unwrap())
                    .build(),
            )
            .build();

        let mut engine = Engine::new(config);
        let handle = engine.start().await.unwrap();

        assert_ne!(handle.address().port(), 0, "should resolve to a real port");
    }
}
