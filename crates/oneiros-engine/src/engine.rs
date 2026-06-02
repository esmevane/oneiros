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
//!
//! Server lifetime is owned by `Server` (`http::server`), not `Engine`.
//! Consumers that need to serve HTTP/MCP construct a `Server` directly:
//! `Server::new(config).serve()` blocks the calling task; `spawn()`
//! returns a handle.

use anstream::{stderr, stdout};
use clap::Parser;
use std::{io::Write, path::Path, process::ExitCode};
use tempfile::TempDir;

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

        let _logging_guard = match Logging.install(engine.config(), cli.command.is_server()) {
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

        // Silent results have already produced their output (e.g. binary
        // bytes streamed to stdout by `storage get`). Skip the render
        // dispatch entirely so we don't append JSON to the stream.
        if result.is_silent() {
            return ExitCode::SUCCESS;
        }

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

    /// From CLI args — parses arguments and resolves configuration.
    ///
    /// Layers config from defaults, config file, env vars, and CLI flags.
    /// Returns the engine and the parsed CLI so the caller can execute
    /// and render. Tracing setup is the caller's responsibility.
    pub(crate) fn from_cli() -> Result<(Self, Cli), Error> {
        let cli = Cli::parse();
        let config = Config::resolve(&cli.overrides).map_err(|e| Error::Config(e.to_string()))?;

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

    /// The resolved configuration.
    pub(crate) fn config(&self) -> &Config {
        &self.config
    }
}

/// Generate the OpenAPI schema for this engine version and write it to
/// `output_path` as JSON.
///
/// Creates a throwaway [`ServerState`] with a temp directory, builds the
/// full router (which populates the API spec via `finish_api_with`), then
/// extracts the compiled OpenAPI document. Used at build time by xtask to
/// feed `@hey-api/openapi-ts` for TypeScript client generation.
///
/// The temp directory is automatically cleaned up when this function
/// returns (the `TempDir` is dropped).
pub fn write_openapi_schema(output_path: impl AsRef<Path>) -> Result<(), Box<dyn core::error::Error>> {
    let temp_dir = TempDir::new()?;
    let config = Config::builder()
        .data_dir(temp_dir.path().to_path_buf())
        .project(ProjectName::new("schema-gen"))
        .service(
            ServiceConfig::builder()
                .address("127.0.0.1:0".parse()?)
                .build(),
        )
        .build();

    let rt = tokio::runtime::Runtime::new()?;
    let api = rt.block_on(async {
        let state = ServerState::bind(config).await?;
        // Building the router populates state.api() via finish_api_with
        let _router = Server::router_from_state(state.clone());
        Ok::<_, Box<dyn core::error::Error>>(state.api().cloned().unwrap_or_default())
    })?;

    let json = serde_json::to_string_pretty(&api)?;
    Platform::default().write(&output_path, &json)?;

    Ok(())
}
