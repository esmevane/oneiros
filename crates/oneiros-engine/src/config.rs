use bon::Builder;
use clap::Parser;
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

/// Configuration for the engine.
///
/// Carries paths, service address, and tuning knobs. Shared between
/// Server (which binds to the address) and Client (which connects to it).
#[derive(Builder, Debug, Clone, Parser, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Root directory for brain data (blobs, exports, etc.)
    #[arg(long, short, global = true, default_value_os_t = default_data_dir())]
    #[builder(default = default_data_dir())]
    pub data_dir: PathBuf,
    /// The brain (project) name. Auto-detected from cwd if not specified.
    #[arg(long, short, global = true, default_value_t = detect_brain_name())]
    #[builder(into, default = detect_brain_name())]
    pub brain: BrainName,
    /// The bookmark (lens) to operate through. Defaults to main.
    #[arg(long, global = true, default_value_t = BookmarkName::main())]
    #[builder(into, default = BookmarkName::main())]
    pub bookmark: BookmarkName,
    /// Service management configuration.
    #[command(flatten)]
    #[builder(default)]
    pub service: ServiceConfig,
    /// Default dream assembly configuration.
    #[command(flatten)]
    #[builder(default)]
    pub dream: DreamConfig,
    /// Default patience window for eventually-consistent reads.
    #[command(flatten)]
    #[builder(default)]
    pub fetch: Fetch,
    /// Output format: prompt (default), json, or text.
    #[arg(long, short, default_value_t, global = true)]
    #[builder(default)]
    pub output: OutputMode,
    /// When to use colored output: auto (default), always, or never.
    #[arg(long, default_value_t, global = true)]
    #[builder(default)]
    pub color: ColorChoice,
    /// How much detail to show: quiet, normal (default), or verbose.
    #[arg(long, default_value_t, global = true)]
    #[builder(default)]
    pub verbosity: Verbosity,
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
            output: OutputMode::default(),
            color: ColorChoice::default(),
            verbosity: Verbosity::default(),
        }
    }
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

    /// A Platform bound to this config's data directory.
    pub fn platform(&self) -> Platform {
        Platform::new(&self.data_dir)
    }

    /// Path to the brain's data directory.
    pub fn brain_dir(&self) -> PathBuf {
        self.platform().brain_dir(&self.brain)
    }

    /// Path to the config file in the data directory.
    pub fn config_path(&self) -> PathBuf {
        self.platform().config_path()
    }

    /// Open the system database.
    pub fn system_db(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        let conn = rusqlite::Connection::open(self.platform().system_db_path())?;
        conn.pragma_update(None, "journal_mode", "wal")?;
        Ok(conn)
    }

    /// Path to the brain's event log database.
    pub fn events_db_path(&self) -> PathBuf {
        self.platform().events_db_path(&self.brain)
    }

    /// Path to the bookmark's projection database.
    pub fn bookmark_db_path(&self) -> PathBuf {
        self.platform()
            .bookmark_db_path(&self.brain, &self.bookmark)
    }

    /// Directory containing all bookmark databases for this brain.
    pub fn bookmarks_dir(&self) -> PathBuf {
        self.platform().bookmarks_dir(&self.brain)
    }

    /// Open the bookmark DB as base with the events DB ATTACHed.
    ///
    /// Unqualified table names resolve to the bookmark DB (projections).
    /// Event log operations use the `events` schema qualifier.
    /// Both share one connection and transaction for atomicity.
    pub fn bookmark_conn(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        let platform = self.platform();
        let _ = platform.ensure_bookmarks_dir(&self.brain);

        let conn =
            rusqlite::Connection::open(platform.bookmark_db_path(&self.brain, &self.bookmark))?;
        conn.pragma_update(None, "journal_mode", "wal")?;
        conn.pragma_update(None, "limit_attached", "125")?;

        conn.execute_batch(&format!(
            "ATTACH DATABASE '{}' AS events",
            platform.events_db_path(&self.brain).display(),
        ))?;

        Ok(conn)
    }

    /// Path to the token file for the current brain.
    pub fn token_path(&self) -> PathBuf {
        self.platform().token_path(&self.brain)
    }

    /// Read the token for the current brain, if one exists.
    pub fn token(&self) -> Option<Token> {
        std::fs::read_to_string(self.token_path())
            .ok()
            .map(|s| Token::from(s.trim()))
    }

    /// Load a config file from `{data_dir}/config.toml` and merge
    /// file values under CLI values.
    ///
    /// File values provide the base; CLI-provided values override them.
    /// If no file exists or is empty, returns self unchanged.
    pub fn with_config_file(mut self) -> Self {
        let path = self.config_path();

        let file_config = match std::fs::read_to_string(&path) {
            Ok(contents) if !contents.trim().is_empty() => {
                match toml::from_str::<Config>(&contents) {
                    Ok(config) => config,
                    Err(err) => {
                        eprintln!(
                            "warning: ignoring malformed config file {}: {err}",
                            path.display()
                        );
                        return self;
                    }
                }
            }
            _ => return self,
        };

        let defaults = Config::default();

        // Merge: if the CLI value matches the default, take the file value.
        // This heuristic cannot distinguish "user explicitly passed the default"
        // from "default was used" — a known limitation until we adopt figment.

        // Top-level
        if self.data_dir == defaults.data_dir {
            self.data_dir = file_config.data_dir;
        }
        if self.brain == defaults.brain {
            self.brain = file_config.brain;
        }
        if self.bookmark == defaults.bookmark {
            self.bookmark = file_config.bookmark;
        }
        if self.output == defaults.output {
            self.output = file_config.output;
        }
        if self.color == defaults.color {
            self.color = file_config.color;
        }
        if self.verbosity == defaults.verbosity {
            self.verbosity = file_config.verbosity;
        }

        // Service
        if self.service.address == defaults.service.address {
            self.service.address = file_config.service.address;
        }
        if self.service.label == defaults.service.label {
            self.service.label = file_config.service.label;
        }
        if self.service.restart_delay_secs == defaults.service.restart_delay_secs {
            self.service.restart_delay_secs = file_config.service.restart_delay_secs;
        }
        if self.service.health_check_delays_ms == defaults.service.health_check_delays_ms {
            self.service.health_check_delays_ms = file_config.service.health_check_delays_ms;
        }

        // Dream
        if self.dream.recent_window == defaults.dream.recent_window {
            self.dream.recent_window = file_config.dream.recent_window;
        }
        if self.dream.dream_depth == defaults.dream.dream_depth {
            self.dream.dream_depth = file_config.dream.dream_depth;
        }
        if self.dream.cognition_size == defaults.dream.cognition_size {
            self.dream.cognition_size = file_config.dream.cognition_size;
        }
        if self.dream.recollection_level == defaults.dream.recollection_level {
            self.dream.recollection_level = file_config.dream.recollection_level;
        }
        if self.dream.recollection_size == defaults.dream.recollection_size {
            self.dream.recollection_size = file_config.dream.recollection_size;
        }
        if self.dream.experience_size == defaults.dream.experience_size {
            self.dream.experience_size = file_config.dream.experience_size;
        }

        // Fetch
        if self.fetch.interval == defaults.fetch.interval {
            self.fetch.interval = file_config.fetch.interval;
        }
        if self.fetch.timeout == defaults.fetch.timeout {
            self.fetch.timeout = file_config.fetch.timeout;
        }

        self
    }

    /// Build a system context from this config.
    pub fn system(&self) -> HostLog {
        HostLog::new(self.clone())
    }

    /// Build a project context from this config.
    pub fn project(&self) -> ProjectLog {
        ProjectLog::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a Config pointing at a tempdir for isolated file tests.
    fn config_in(dir: &std::path::Path) -> Config {
        Config::builder()
            .data_dir(dir.to_path_buf())
            .brain(BrainName::new("test"))
            .build()
    }

    fn write_config(dir: &std::path::Path, contents: &str) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join("config.toml"), contents).unwrap();
    }

    #[test]
    fn missing_file_returns_config_unchanged() {
        let dir = tempfile::tempdir().unwrap();
        let config = config_in(dir.path());
        let merged = config.clone().with_config_file();

        assert_eq!(merged.service.address, config.service.address);
        assert_eq!(merged.dream.cognition_size, config.dream.cognition_size);
        assert_eq!(merged.output, config.output);
    }

    #[test]
    fn empty_file_returns_config_unchanged() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), "");
        let config = config_in(dir.path());
        let merged = config.clone().with_config_file();

        assert_eq!(merged.service.address, config.service.address);
    }

    #[test]
    fn malformed_file_returns_config_unchanged() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), "this is not valid toml {{{{");
        let config = config_in(dir.path());
        let merged = config.clone().with_config_file();

        assert_eq!(merged.service.address, config.service.address);
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
        let config = config_in(dir.path());
        let merged = config.with_config_file();

        assert_eq!(
            merged.service.address,
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
        let config = config_in(dir.path());
        let merged = config.with_config_file();

        assert_eq!(merged.fetch.interval, std::time::Duration::from_millis(50));
        assert_eq!(merged.fetch.timeout, std::time::Duration::from_secs(5));
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
        let config = config_in(dir.path());
        let merged = config.with_config_file();

        assert_eq!(merged.dream.cognition_size, 50);
        assert_eq!(merged.dream.dream_depth, 3);
        // Unspecified fields keep their defaults
        assert_eq!(merged.dream.recent_window, 5);
        assert_eq!(merged.dream.recollection_size, 30);
    }

    #[test]
    fn cli_override_survives_file_merge() {
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
        let mut config = config_in(dir.path());
        // Simulate CLI setting a non-default value
        config.service.address = "127.0.0.1:9999".parse().unwrap();
        config.dream.cognition_size = 5;

        let merged = config.with_config_file();

        // CLI values survive — they differ from defaults
        assert_eq!(
            merged.service.address,
            "127.0.0.1:9999".parse::<SocketAddr>().unwrap()
        );
        assert_eq!(merged.dream.cognition_size, 5);
    }

    #[test]
    fn file_overrides_output_mode() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), r#"output = "json""#);
        let config = config_in(dir.path());
        let merged = config.with_config_file();

        assert_eq!(merged.output, OutputMode::Json);
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
        let config = config_in(dir.path());
        let merged = config.with_config_file();

        assert_eq!(merged.service.health_check_delays_ms, vec![100, 200]);
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
        let config = config_in(dir.path());
        let merged = config.with_config_file();

        assert_eq!(merged.dream.experience_size, 25);
        assert_eq!(merged.dream.cognition_size, 20); // default preserved
        assert_eq!(merged.dream.dream_depth, 1); // default preserved
    }

    #[test]
    fn file_overrides_color_choice() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), r#"color = "never""#);
        let config = config_in(dir.path());
        let merged = config.with_config_file();

        assert_eq!(merged.color, ColorChoice::Never);
    }

    #[test]
    fn file_overrides_verbosity() {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), r#"verbosity = "verbose""#);
        let config = config_in(dir.path());
        let merged = config.with_config_file();

        assert_eq!(merged.verbosity, Verbosity::Verbose);
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
        let config = config_in(dir.path());
        // Should warn and return unchanged (typo "sevice" is unknown)
        let merged = config.clone().with_config_file();

        assert_eq!(merged.service.address, config.service.address);
    }
}
