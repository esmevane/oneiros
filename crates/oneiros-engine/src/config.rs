use bon::Builder;
use clap::{Args, Parser};
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};
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
    Platform::default().data_dir().to_path_buf()
}

// ── CLI Override Types ─────────────────────────────────────────────

/// CLI overrides for service configuration.
///
/// Every field is optional — only `Some` values override the resolved
/// configuration. Flattened into [`CliOverrides`] so flags appear at
/// the top level (e.g. `--address`, not `--service-address`).
#[derive(Args, Debug, Clone, Serialize, Default)]
pub(crate) struct ServiceCli {
    /// The service label for OS registration.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) label: Option<String>,
    /// Address the service listens on.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) address: Option<SocketAddr>,
    /// URL scheme for HTTP clients: "http" or "https".
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) scheme: Option<String>,
    /// Health check retry delays after starting (milliseconds).
    #[arg(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) health_check_delays_ms: Option<Vec<u64>>,
    /// Restart delay on failure (seconds).
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) restart_delay_secs: Option<u32>,
}

/// CLI overrides for dream assembly configuration.
///
/// Every field is optional — only `Some` values override the resolved
/// configuration. Flattened into [`CliOverrides`] so flags appear at
/// the top level (e.g. `--cognition-size`, not `--dream-cognition-size`).
#[derive(Args, Debug, Clone, Serialize, Default)]
pub(crate) struct DreamCli {
    /// Number of recent cognitions and experiences in the orientation window.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) recent_window: Option<usize>,
    /// Maximum BFS traversal depth from the seed set.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dream_depth: Option<usize>,
    /// Maximum number of cognitions in the dream.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cognition_size: Option<usize>,
    /// Minimum memory level to include (log-level semantics).
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) recollection_level: Option<LevelName>,
    /// Maximum number of non-core memories in the dream.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) recollection_size: Option<usize>,
    /// Maximum number of experiences in the dream.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) experience_size: Option<usize>,
}

/// CLI overrides for fetch/polling configuration.
///
/// Every field is optional — only `Some` values override the resolved
/// configuration. Flattened into [`CliOverrides`] so flags appear at
/// the top level (e.g. `--interval`, not `--fetch-interval`).
#[derive(Args, Debug, Clone, Serialize, Default)]
pub(crate) struct FetchCli {
    /// How often to poll while waiting for an eventually-consistent read.
    #[arg(long, global = true, value_parser = humantime::parse_duration)]
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_duration_option"
    )]
    pub(crate) interval: Option<std::time::Duration>,
    /// Maximum time to wait before giving up.
    #[arg(long, global = true, value_parser = humantime::parse_duration)]
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_duration_option"
    )]
    pub(crate) timeout: Option<std::time::Duration>,
}

/// Serialize an `Option<Duration>` as a humantime string for Figment.
fn serialize_duration_option<S: serde::Serializer>(
    value: &Option<std::time::Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(d) => serializer.serialize_str(&humantime::format_duration(*d).to_string()),
        None => serializer.serialize_none(),
    }
}

/// CLI overrides for database tuning.
#[derive(Args, Debug, Clone, Serialize, Default)]
pub(crate) struct DatabaseCli {
    /// Maximum concurrently-attached databases (SQLite pragma).
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limit_attached: Option<u32>,
}

impl DatabaseCli {
    fn is_empty(&self) -> bool {
        self.limit_attached.is_none()
    }
}

/// CLI overrides for general system configuration.
#[derive(Args, Debug, Clone, Serialize, Default)]
pub(crate) struct GeneralCli {
    /// Default page size for paginated lists.
    /// Not exposed as a CLI flag — config file / env var only.
    #[arg(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) default_page_size: Option<usize>,
}

impl GeneralCli {
    fn is_empty(&self) -> bool {
        self.default_page_size.is_none()
    }
}

/// CLI overrides — captures only explicitly-set flags.
///
/// Fields are `Option<T>`: `None` means "not set on CLI", `Some` means
/// "user explicitly provided this value". Figment layers these on top
/// of defaults, config file, and env vars, so CLI always wins.
///
/// This struct replaces `Config`'s former dual role as both CLI parser
/// and resolved configuration. The separation fixes the long-standing
/// "can't distinguish user default from clap default" limitation.
#[derive(Parser, Debug, Clone, Serialize, Default)]
pub(crate) struct CliOverrides {
    /// Root directory for brain data.
    #[arg(long, short, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data_dir: Option<PathBuf>,
    /// The brain (project) name.
    #[arg(long, short, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) brain: Option<BrainName>,
    /// The bookmark (lens) to operate through.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bookmark: Option<BookmarkName>,
    /// Service management configuration overrides.
    #[command(flatten)]
    #[serde(skip_serializing_if = "ServiceCli::is_empty")]
    pub(crate) service: ServiceCli,
    /// Dream assembly configuration overrides.
    #[command(flatten)]
    #[serde(skip_serializing_if = "DreamCli::is_empty")]
    pub(crate) dream: DreamCli,
    /// Fetch/polling configuration overrides.
    #[command(flatten)]
    #[serde(skip_serializing_if = "FetchCli::is_empty")]
    pub(crate) fetch: FetchCli,
    /// Database tuning overrides.
    #[command(flatten)]
    #[serde(skip_serializing_if = "DatabaseCli::is_empty")]
    pub(crate) database: DatabaseCli,
    /// General system configuration overrides.
    #[command(flatten)]
    #[serde(skip_serializing_if = "GeneralCli::is_empty")]
    pub(crate) general: GeneralCli,
    /// Output format: prompt, json, or text.
    #[arg(long, short, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output: Option<OutputMode>,
    /// When to use colored output: auto, always, or never.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) color: Option<ColorChoice>,
    /// How much detail to show: quiet, normal, or verbose.
    #[arg(long, global = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verbosity: Option<Verbosity>,
}

impl ServiceCli {
    fn is_empty(&self) -> bool {
        self.label.is_none()
            && self.address.is_none()
            && self.scheme.is_none()
            && self.health_check_delays_ms.is_none()
            && self.restart_delay_secs.is_none()
    }
}

impl DreamCli {
    fn is_empty(&self) -> bool {
        self.recent_window.is_none()
            && self.dream_depth.is_none()
            && self.cognition_size.is_none()
            && self.recollection_level.is_none()
            && self.recollection_size.is_none()
            && self.experience_size.is_none()
    }
}

impl FetchCli {
    fn is_empty(&self) -> bool {
        self.interval.is_none() && self.timeout.is_none()
    }
}

// ── Resolved Configuration ────────────────────────────────────────

/// Configuration for the engine.
///
/// Carries paths, service address, and tuning knobs. Shared between
/// Server (which binds to the address) and Client (which connects to it).
///
/// Constructed via [`Config::builder()`] (for tests and programmatic use)
/// or [`Config::resolve()`] (from CLI, config file, and env vars).
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Config {
    /// Root directory for brain data (blobs, exports, etc.)
    #[builder(default = default_data_dir())]
    pub(crate) data_dir: PathBuf,
    /// The brain (project) name. Auto-detected from cwd if not specified.
    #[builder(into, default = detect_brain_name())]
    pub(crate) brain: BrainName,
    /// The bookmark (lens) to operate through. Defaults to main.
    #[builder(into, default = BookmarkName::main())]
    pub(crate) bookmark: BookmarkName,
    /// Service management configuration.
    #[builder(default)]
    pub(crate) service: ServiceConfig,
    /// Default dream assembly configuration.
    #[builder(default)]
    pub(crate) dream: DreamConfig,
    /// Default patience window for eventually-consistent reads.
    #[builder(default)]
    pub(crate) fetch: Fetch,
    /// Database tuning knobs.
    #[builder(default)]
    pub(crate) database: DatabaseConfig,
    /// General system configuration.
    #[builder(default)]
    pub(crate) general: GeneralConfig,
    /// Output format: prompt (default), json, or text.
    #[builder(default)]
    pub(crate) output: OutputMode,
    /// When to use colored output: auto (default), always, or never.
    #[builder(default)]
    pub(crate) color: ColorChoice,
    /// How much detail to show: quiet, normal (default), or verbose.
    #[builder(default)]
    pub(crate) verbosity: Verbosity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: default_data_dir(),
            brain: detect_brain_name(),
            bookmark: BookmarkName::main(),
            service: ServiceConfig::default(),
            dream: DreamConfig::default(),
            fetch: Fetch::default(),
            database: DatabaseConfig::default(),
            general: GeneralConfig::default(),
            output: OutputMode::default(),
            color: ColorChoice::default(),
            verbosity: Verbosity::default(),
        }
    }
}

impl Config {
    /// The service address (convenience accessor).
    pub(crate) fn service_addr(&self) -> SocketAddr {
        self.service.address
    }

    /// The base URL for HTTP clients to connect to the service.
    pub(crate) fn base_url(&self) -> String {
        format!("{}://{}", self.service.scheme, self.service.address)
    }

    /// A Platform bound to this config's data directory.
    pub(crate) fn platform(&self) -> Platform {
        Platform::new(&self.data_dir)
    }

    /// Open the system database.
    pub(crate) fn system_db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        let conn = rusqlite::Connection::open(self.platform().system_db_path())?;
        conn.pragma_update(None, "journal_mode", "wal")?;
        Ok(conn)
    }

    /// Path to the brain's event log database.
    pub(crate) fn events_db_path(&self) -> PathBuf {
        self.platform().events_db_path(&self.brain)
    }

    /// Path to the bookmark's projection database.
    pub(crate) fn bookmark_db_path(&self) -> PathBuf {
        self.platform()
            .bookmark_db_path(&self.brain, &self.bookmark)
    }

    /// Directory containing all bookmark databases for this brain.
    pub(crate) fn bookmarks_dir(&self) -> PathBuf {
        self.platform().bookmarks_dir(&self.brain)
    }

    /// Open the bookmark DB as base with the events DB ATTACHed.
    ///
    /// Unqualified table names resolve to the bookmark DB (projections).
    /// Event log operations use the `events` schema qualifier.
    /// Both share one connection and transaction for atomicity.
    pub(crate) fn bookmark_conn(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        let platform = self.platform();
        let _ = platform.ensure_bookmarks_dir(&self.brain);

        let conn =
            rusqlite::Connection::open(platform.bookmark_db_path(&self.brain, &self.bookmark))?;
        conn.pragma_update(None, "journal_mode", "wal")?;
        conn.pragma_update(
            None,
            "limit_attached",
            self.database.limit_attached.to_string(),
        )?;

        conn.execute_batch(&format!(
            "ATTACH DATABASE '{}' AS events",
            platform.events_db_path(&self.brain).display(),
        ))?;

        Ok(conn)
    }

    /// Read the token for the current brain, if one exists.
    pub(crate) fn token(&self) -> Option<Token> {
        let platform = self.platform();
        platform
            .read_to_string(platform.token_path(&self.brain))
            .ok()
            .map(|s| Token::from(s.trim()))
    }

    /// Resolve configuration from layered sources.
    ///
    /// Merges in priority order (lowest to highest):
    ///
    /// 1. Rust hardcoded defaults ([`Config::default`])
    /// 2. `{data_dir}/config.toml`
    /// 3. `ONEIROS_`-prefixed environment variables (double-underscore
    ///    separates nested keys, e.g. `ONEIROS_SERVICE__ADDRESS`)
    /// 4. CLI flags from `overrides` — only `Some` fields apply
    ///
    /// If the config file is missing or empty, defaults carry through.
    /// Malformed files produce a warning on stderr and are ignored.
    ///
    /// # Errors
    ///
    /// Returns [`figment::Error`] if the assembled configuration fails
    /// to deserialize into [`Config`] (type mismatches, missing required
    /// fields, etc.).
    ///
    /// `figment::Error` is large (~200 bytes); the `#[allow]` avoids
    /// the `clippy::result_large_err` lint since boxing would break
    /// the caller ergonomics.
    #[allow(clippy::result_large_err)]
    pub(crate) fn resolve(overrides: &CliOverrides) -> Result<Self, figment::Error> {
        let data_dir = overrides.data_dir.clone().unwrap_or_else(default_data_dir);
        let config_path = Platform::new(&data_dir).config_path();

        let defaults = Config::builder().data_dir(data_dir).build();

        let figment = Figment::new()
            .merge(Serialized::defaults(defaults.clone()))
            .merge(Toml::file(&config_path))
            .merge(Env::prefixed("ONEIROS_").split("__"))
            .merge(Serialized::defaults(overrides));

        match figment.extract::<Self>() {
            Ok(config) => Ok(config),
            Err(err) => {
                let is_toml_syntax_error = matches!(&err.kind, figment::error::Kind::Message(msg)
                    if msg.contains("parse error") || msg.contains("expected"));
                let from_toml = err
                    .metadata
                    .as_ref()
                    .is_some_and(|m| m.name.contains("TOML"));
                let path_exists = config_path.exists();

                if is_toml_syntax_error && from_toml && path_exists {
                    eprintln!(
                        "warning: ignoring malformed config file {}: {err}",
                        config_path.display()
                    );
                    // Retry without the file provider, using the same defaults
                    Figment::new()
                        .merge(Serialized::defaults(defaults))
                        .merge(Env::prefixed("ONEIROS_").split("__"))
                        .merge(Serialized::defaults(overrides))
                        .extract()
                } else {
                    Err(err)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build overrides with everything at default (all `None`).
    fn empty_overrides() -> CliOverrides {
        CliOverrides::default()
    }

    /// Build a Config pointing at a tempdir for isolated file tests.
    fn config_in(dir: &std::path::Path) -> Config {
        Config::builder()
            .data_dir(dir.to_path_buf())
            .brain(BrainName::new("test"))
            .build()
    }

    fn write_config(dir: &std::path::Path, contents: &str) {
        let platform = Platform::new(dir);
        platform.ensure_dir(platform.data_dir()).unwrap();
        platform.write(platform.config_path(), contents).unwrap();
    }

    /// Resolve with explicit overrides, setting data_dir from the dir.
    fn resolve_in(dir: &std::path::Path, overrides: &CliOverrides) -> Config {
        let mut ov = overrides.clone();
        if ov.data_dir.is_none() {
            ov.data_dir = Some(dir.to_path_buf());
        }
        Config::resolve(&ov).expect("config resolution should succeed")
    }

    #[test]
    fn missing_file_returns_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let config = resolve_in(dir.path(), &empty_overrides());

        let expected = config_in(dir.path());
        assert_eq!(config.service.address, expected.service.address);
        assert_eq!(config.dream.cognition_size, expected.dream.cognition_size);
        assert_eq!(config.output, expected.output);
    }

    #[test]
    fn empty_file_returns_defaults() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), "");
        let config = resolve_in(dir.path(), &empty_overrides());

        let expected = config_in(dir.path());
        assert_eq!(config.service.address, expected.service.address);
    }

    #[test]
    fn malformed_file_falls_back_to_defaults() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), "this is not valid toml {{{{");
        let config = resolve_in(dir.path(), &empty_overrides());

        let expected = config_in(dir.path());
        assert_eq!(config.service.address, expected.service.address);
    }

    #[test]
    fn file_overrides_default_service_address() {
        let dir = tempfile::tempdir().unwrap();
        write_config(
            dir.path(),
            r#"
[service]
address = "127.0.0.1:3000"
"#,
        );
        let config = resolve_in(dir.path(), &empty_overrides());

        assert_eq!(
            config.service.address,
            "127.0.0.1:3000".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn file_overrides_default_fetch_config() {
        let dir = tempfile::tempdir().unwrap();
        write_config(
            dir.path(),
            r#"
[fetch]
interval = "50ms"
timeout = "5s"
"#,
        );
        let config = resolve_in(dir.path(), &empty_overrides());

        assert_eq!(config.fetch.interval, std::time::Duration::from_millis(50));
        assert_eq!(config.fetch.timeout, std::time::Duration::from_secs(5));
    }

    #[test]
    fn file_overrides_default_dream_config() {
        let dir = tempfile::tempdir().unwrap();
        write_config(
            dir.path(),
            r#"
[dream]
cognition_size = 50
dream_depth = 3
"#,
        );
        let config = resolve_in(dir.path(), &empty_overrides());

        assert_eq!(config.dream.cognition_size, 50);
        assert_eq!(config.dream.dream_depth, 3);
        // Unspecified fields keep their defaults
        assert_eq!(config.dream.recent_window, 5);
        assert_eq!(config.dream.recollection_size, 30);
    }

    #[test]
    fn cli_overrides_file_and_defaults() {
        let dir = tempfile::tempdir().unwrap();
        write_config(
            dir.path(),
            r#"
[service]
address = "127.0.0.1:3000"

[dream]
cognition_size = 50
"#,
        );

        let overrides = CliOverrides {
            service: ServiceCli {
                address: Some("127.0.0.1:9999".parse().unwrap()),
                ..Default::default()
            },
            dream: DreamCli {
                cognition_size: Some(5),
                ..Default::default()
            },
            data_dir: Some(dir.path().to_path_buf()),
            ..Default::default()
        };

        let config = Config::resolve(&overrides).expect("resolve should succeed");

        // CLI values win over file
        assert_eq!(
            config.service.address,
            "127.0.0.1:9999".parse::<SocketAddr>().unwrap()
        );
        assert_eq!(config.dream.cognition_size, 5);
    }

    #[test]
    fn partial_cli_overrides_let_file_values_through() {
        let dir = tempfile::tempdir().unwrap();
        write_config(
            dir.path(),
            r#"
[dream]
cognition_size = 50
dream_depth = 3
"#,
        );

        // Only set cognition_size via CLI — dream_depth should come from file
        let overrides = CliOverrides {
            dream: DreamCli {
                cognition_size: Some(5),
                ..Default::default()
            },
            data_dir: Some(dir.path().to_path_buf()),
            ..Default::default()
        };

        let config = Config::resolve(&overrides).expect("resolve should succeed");

        assert_eq!(config.dream.cognition_size, 5); // CLI
        assert_eq!(config.dream.dream_depth, 3); // from file
    }

    #[test]
    fn file_overrides_output_mode() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), r#"output = "json""#);
        let config = resolve_in(dir.path(), &empty_overrides());

        assert_eq!(config.output, OutputMode::Json);
    }

    #[test]
    fn file_overrides_health_check_delays() {
        let dir = tempfile::tempdir().unwrap();
        write_config(
            dir.path(),
            r#"
[service]
health_check_delays_ms = [100, 200]
"#,
        );
        let config = resolve_in(dir.path(), &empty_overrides());

        assert_eq!(config.service.health_check_delays_ms, vec![100, 200]);
    }

    #[test]
    fn partial_dream_section_fills_defaults() {
        let dir = tempfile::tempdir().unwrap();
        write_config(
            dir.path(),
            r#"
[dream]
experience_size = 25
"#,
        );
        let config = resolve_in(dir.path(), &empty_overrides());

        assert_eq!(config.dream.experience_size, 25);
        assert_eq!(config.dream.cognition_size, 20); // default preserved
        assert_eq!(config.dream.dream_depth, 1); // default preserved
    }

    #[test]
    fn file_overrides_color_choice() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), r#"color = "never""#);
        let config = resolve_in(dir.path(), &empty_overrides());

        assert_eq!(config.color, ColorChoice::Never);
    }

    #[test]
    fn file_overrides_verbosity() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), r#"verbosity = "verbose""#);
        let config = resolve_in(dir.path(), &empty_overrides());

        assert_eq!(config.verbosity, Verbosity::Verbose);
    }

    #[test]
    fn unknown_field_in_config_file_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        write_config(
            dir.path(),
            r#"
[sevice]
address = "127.0.0.1:3000"
"#,
        );
        // Figment with deny_unknown_fields on Config should reject unknown keys
        let overrides = CliOverrides {
            data_dir: Some(dir.path().to_path_buf()),
            ..Default::default()
        };
        let result = Config::resolve(&overrides);
        assert!(
            result.is_err(),
            "unknown field 'sevice' should cause an error"
        );
    }

    #[test]
    fn cli_data_dir_determines_config_file_location() {
        let dir_b = tempfile::tempdir().unwrap();

        // Config file in dir_b overrides service address
        write_config(
            dir_b.path(),
            r#"
[service]
address = "127.0.0.1:4000"
"#,
        );

        let overrides = CliOverrides {
            data_dir: Some(dir_b.path().to_path_buf()),
            ..Default::default()
        };

        let config = Config::resolve(&overrides).expect("resolve should succeed");
        assert_eq!(
            config.service.address,
            "127.0.0.1:4000".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn env_var_overrides_file() {
        use figment::Jail;

        #[expect(clippy::result_large_err)]
        Jail::expect_with(|jail| {
            jail.create_file("config.toml", r#"output = "json""#)?;
            jail.set_env("ONEIROS_OUTPUT", "prompt");

            // Resolve using Jail's isolated filesystem and env
            let figment = Figment::new()
                .merge(Serialized::defaults(Config::default()))
                .merge(Toml::file("config.toml"))
                .merge(Env::prefixed("ONEIROS_").split("__"))
                .merge(Serialized::defaults(empty_overrides()));

            let config: Config = figment.extract()?;
            assert_eq!(config.output, OutputMode::Prompt);

            Ok(())
        });
    }

    #[test]
    fn two_way_coverage_config_and_overrides_match() {
        // This test ensures CliOverrides covers every Config field.
        // If a field is added to Config but not CliOverrides, this
        // test won't compile (field missing from CliOverrides struct).
        // If a field is added to CliOverrides but not Config, serde's
        // deny_unknown_fields on Config catches it at runtime.

        let dir = tempfile::tempdir().unwrap();

        let overrides = CliOverrides {
            data_dir: Some(dir.path().to_path_buf()),
            brain: Some(BrainName::new("coverage-brain")),
            bookmark: Some(BookmarkName::new("coverage-lens")),
            service: ServiceCli {
                label: Some("com.test.coverage".into()),
                address: Some("127.0.0.1:7777".parse().unwrap()),
                scheme: Some("https".into()),
                health_check_delays_ms: Some(vec![50, 100]),
                restart_delay_secs: Some(10),
            },
            dream: DreamCli {
                recent_window: Some(42),
                dream_depth: Some(7),
                cognition_size: Some(99),
                recollection_level: Some(LevelName::new("session")),
                recollection_size: Some(88),
                experience_size: Some(77),
            },
            fetch: FetchCli {
                interval: Some(std::time::Duration::from_millis(123)),
                timeout: Some(std::time::Duration::from_secs(456)),
            },
            database: DatabaseCli {
                limit_attached: Some(256),
            },
            general: GeneralCli {
                default_page_size: Some(50),
            },
            output: Some(OutputMode::Text),
            color: Some(ColorChoice::Always),
            verbosity: Some(Verbosity::Verbose),
        };

        let config = Config::resolve(&overrides).expect("full coverage resolve should succeed");

        // Every field from overrides should land in the resolved config
        assert_eq!(config.data_dir, dir.path().to_path_buf());
        assert_eq!(config.brain, BrainName::new("coverage-brain"));
        assert_eq!(config.bookmark, BookmarkName::new("coverage-lens"));
        assert_eq!(config.service.label, "com.test.coverage");
        assert_eq!(
            config.service.address,
            "127.0.0.1:7777".parse::<SocketAddr>().unwrap()
        );
        assert_eq!(config.service.health_check_delays_ms, vec![50, 100]);
        assert_eq!(config.service.restart_delay_secs, 10);
        assert_eq!(config.service.scheme, "https");
        assert_eq!(config.dream.recent_window, 42);
        assert_eq!(config.dream.dream_depth, 7);
        assert_eq!(config.dream.cognition_size, 99);
        assert_eq!(config.dream.recollection_level, LevelName::new("session"));
        assert_eq!(config.dream.recollection_size, 88);
        assert_eq!(config.dream.experience_size, 77);
        assert_eq!(config.fetch.interval, std::time::Duration::from_millis(123));
        assert_eq!(config.fetch.timeout, std::time::Duration::from_secs(456));
        assert_eq!(config.database.limit_attached, 256);
        assert_eq!(config.general.default_page_size, 50);
        assert_eq!(config.output, OutputMode::Text);
        assert_eq!(config.color, ColorChoice::Always);
        assert_eq!(config.verbosity, Verbosity::Verbose);
    }
}
