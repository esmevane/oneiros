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
use aide::openapi::{Parameter, ParameterData, ParameterSchemaOrContent, PathStyle, ReferenceOr, SchemaObject};
use clap::Parser;
use schemars::json_schema;
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
    let mut api = rt.block_on(async {
        let state = ServerState::bind(config).await?;
        // Building the router populates state.api() via finish_api_with
        let _router = Server::router_from_state(state.clone());
        Ok::<_, Box<dyn core::error::Error>>(state.api().cloned().unwrap_or_default())
    })?;

    // Path parameters from URL templates (e.g. /cognitions/{id}) aren't
    // automatically documented when extractor types use generic wrappers
    // like ResourceKey<K>. Patch them in.
    patch_path_parameters(&mut api);

    let json = serde_json::to_string_pretty(&api)?;
    Platform::default().write(&output_path, &json)?;

    Ok(())
}

/// Extract path parameter names from a URL template string.
///
/// Returns each `{name}` placeholder found in the path, e.g.
/// `"/cognitions/{id}"` yields `["id"]`.
fn extract_path_params(path: &str) -> Vec<&str> {
    let mut params = Vec::new();
    let mut start = None;
    for (i, ch) in path.char_indices() {
        match ch {
            '{' => start = Some(i + 1),
            '}' => {
                if let Some(s) = start.take() {
                    params.push(&path[s..i]);
                }
            }
            _ => {}
        }
    }
    params
}

/// Ensure that every path with a `{param}` placeholder has a corresponding
/// OpenAPI path parameter documented on each operation.
///
/// Aide's automatic parameter inference relies on the handler extractor type's
/// JSON schema. When the extractor uses a generic `ResourceKey<K>` (which
/// serializes as a plain string), there are no named properties to extract.
/// This function fills the gap by walking the generated OpenAPI spec and adding
/// missing path parameters for any `{name}` template variables.
fn patch_path_parameters(api: &mut aide::openapi::OpenApi) {
    let Some(paths) = api.paths.as_mut() else {
        return;
    };

    for (path_str, path_ref) in paths.paths.iter_mut() {
        let param_names = extract_path_params(path_str);
        if param_names.is_empty() {
            continue;
        }

        let ReferenceOr::Item(path_item) = path_ref else {
            continue;
        };

        let operations: [Option<&mut aide::openapi::Operation>; 6] = [
            path_item.get.as_mut(),
            path_item.put.as_mut(),
            path_item.post.as_mut(),
            path_item.delete.as_mut(),
            path_item.options.as_mut(),
            path_item.patch.as_mut(),
        ];

        for operation in operations.into_iter().flatten() {
            for &name in &param_names {
                let already_has = operation.parameters.iter().any(|p| match p {
                    ReferenceOr::Item(p) => p.parameter_data_ref().name == name,
                    ReferenceOr::Reference { .. } => false,
                });
                if already_has {
                    continue;
                }

                operation.parameters.push(ReferenceOr::Item(Parameter::Path {
                    parameter_data: ParameterData {
                        name: name.to_string(),
                        description: None,
                        required: true,
                        format: ParameterSchemaOrContent::Schema(SchemaObject {
                            json_schema: json_schema!({"type": "string"}),
                            example: None,
                            external_docs: None,
                        }),
                        deprecated: None,
                        example: None,
                        examples: Default::default(),
                        explode: None,
                        extensions: Default::default(),
                    },
                    style: PathStyle::Simple,
                }));
            }
        }
    }
}
