//! Engine — the formal consumer surface for oneiros.
//!
//! Every consumer of the engine (binary, tests, xtask, programmatic)
//! goes through this type. It owns a `Config` and provides entry points
//! for each use case:
//!
//! - `run()` — full CLI lifecycle: parse, configure, execute, render
//! - `from_cli()` — parse CLI args and merge config file
//! - `new(config)` — explicit config (tests, programmatic use)
//! - `execute(cli)` — run a parsed CLI command
//! - `start()` — bind and serve HTTP/MCP
//! - `package(target)` — emit skill assets to a directory

use anstream::{stderr, stdout};
use clap::Parser;
use std::{io::Write, net::SocketAddr, process::ExitCode};
use tokio::net::TcpListener;

use crate::*;

/// The engine — entry point for all consumers.
pub struct Engine {
    config: Config,
}

impl Engine {
    /// Run the full CLI lifecycle — parse, configure, execute, render.
    ///
    /// This is the canonical entrypoint for the binary. It owns
    /// tracing setup, color configuration, command execution, and
    /// output rendering. Errors are rendered through `ErrorView`
    /// to stderr with styled formatting and proper exit codes.
    pub async fn run() -> ExitCode {
        let (engine, cli) = match Self::from_cli() {
            Ok(pair) => pair,
            Err(error) => {
                let _ = writeln!(stderr().lock(), "{}", ErrorView::new(error));
                return ExitCode::FAILURE;
            }
        };

        engine.config().color.apply_global();

        let _logging_guard = match Logging.install(engine.config()) {
            Ok(guard) => guard,
            Err(error) => {
                let _ = writeln!(stderr().lock(), "{}", ErrorView::new(error.into()));
                return ExitCode::FAILURE;
            }
        };

        let result: Rendered<Responses> = match engine.execute(&cli).await {
            Ok(rendered) => rendered,
            Err(error) => {
                let _ = writeln!(stderr().lock(), "{}", ErrorView::new(error));
                return ExitCode::FAILURE;
            }
        };

        let as_json = match serde_json::to_string(result.response()) {
            Ok(json) => json,
            Err(error) => {
                let _ = writeln!(stderr().lock(), "{}", ErrorView::new(error.into()));
                return ExitCode::FAILURE;
            }
        };

        let mut out = stdout().lock();

        let write_result = match (
            &engine.config().output,
            result.has_prompt(),
            result.has_text(),
        ) {
            (OutputMode::Prompt, true, _) => write!(out, "{}", result.prompt()),
            (OutputMode::Text, _, true) => write!(out, "{}", result.text()),
            (OutputMode::Json, _, _) | (_, false, _) | (_, _, false) => {
                writeln!(out, "{as_json}")
            }
        };

        if let Err(error) = write_result {
            let _ = writeln!(stderr().lock(), "{}", ErrorView::new(error.into()));
            return ExitCode::FAILURE;
        }

        ExitCode::SUCCESS
    }

    /// From CLI args — parses arguments and merges config file.
    ///
    /// Returns the engine and the parsed CLI so the caller can execute
    /// and render. Tracing setup is the caller's responsibility.
    pub(crate) fn from_cli() -> Result<(Self, Cli), Error> {
        let cli = Cli::parse();
        let config = cli.config().clone().with_config_file();

        Ok((Self::new(config), cli))
    }

    /// From explicit config — tests and programmatic consumers.
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    /// Execute a parsed CLI command against this engine's config.
    pub(crate) async fn execute(&self, cli: &Cli) -> Result<Rendered<Responses>, Error> {
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
            if let Err(err) = server.serve(listener).await {
                eprintln!("server exited with error: {err}");
            }
        });

        Ok(ServerHandle { address, handle })
    }

    /// The resolved configuration.
    pub(crate) fn config(&self) -> &Config {
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
