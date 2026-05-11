use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    EnvFilter, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::*;

/// Resolve the tracing filter directives for a given verbosity and surface.
///
/// Surfaces differ in what counts as "default noise." The server is a
/// long-running process where info-level spans are the expected audit
/// trail; the CLI is a short-lived client where the same spans are pure
/// noise to the human at the terminal.
///
/// `aide` is muted off-info regardless because its startup chatter is
/// never useful in either surface.
pub(crate) fn level(verbosity: &Verbosity, is_server: bool) -> &'static str {
    match (verbosity, is_server) {
        (Verbosity::Quiet, true) => "warn,aide=warn",
        (Verbosity::Normal, true) => "info,aide=warn",
        (Verbosity::Verbose, true) => "debug,aide=warn",
        (Verbosity::Quiet, false) => "off",
        (Verbosity::Normal, false) => "off",
        (Verbosity::Verbose, false) => "info,aide=warn",
    }
}

pub(crate) struct Logging;

impl Logging {
    /// Install the global tracing subscriber.
    ///
    /// Hold the returned guard until the process exits. Dropping it flushes
    /// any pending file writes. `RUST_LOG` overrides the surface default.
    pub(crate) fn install(&self, config: &Config, is_server: bool) -> std::io::Result<WorkerGuard> {
        let (file_writer, guard) = tracing_appender::non_blocking(DailyLog::new(&config.data_dir)?);

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(level(&config.verbosity, is_server)));

        let stderr_layer = tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_span_events(FmtSpan::CLOSE)
            .compact();

        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(file_writer)
            .with_span_events(FmtSpan::CLOSE)
            .json();

        tracing_subscriber::registry()
            .with(filter)
            .with(stderr_layer)
            .with(file_layer)
            .init();

        Ok(guard)
    }
}

/// A simple daily-rotating log writer that names files as `oneiros-YYYY-MM-DD.log`.
///
/// Rotation happens lazily on write when the local date changes. Uses
/// append-mode file opens so restarts during the same day continue an
/// existing file rather than truncating.
struct DailyLog {
    date: Timestamp,
    path: PathBuf,
    logs: PathBuf,
    file: Option<File>,
}

impl DailyLog {
    pub(crate) fn new(root: &Path) -> Result<Self, std::io::Error> {
        let mut new_daily_log = Self {
            date: Timestamp::now(),
            logs: root.join("logs").clone(),
            path: PathBuf::default(),
            file: None,
        };

        std::fs::create_dir_all(&new_daily_log.logs)?;

        new_daily_log.construct_path();
        new_daily_log.set_file()?;

        Ok(new_daily_log)
    }

    fn construct_path(&mut self) {
        self.path = self.logs.join(format!(
            "oneiros-{date}.log",
            date = self.date.as_date_string()
        ));
    }

    fn set_file(&mut self) -> Result<(), std::io::Error> {
        self.file = Some(
            std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&self.path)?,
        );

        Ok(())
    }
}

impl Write for DailyLog {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let today = Timestamp::now();
        if self.date.as_date_string() != today.as_date_string() {
            self.date = today;
            self.construct_path();
            self.set_file()?;
        }

        if let Some(file) = &mut self.file {
            file.write(buf)
        } else {
            Ok(0)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(file) = &mut self.file {
            file.flush()
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_default_emits_info() {
        assert_eq!(level(&Verbosity::Normal, true), "info,aide=warn");
    }

    #[test]
    fn server_quiet_drops_to_warn() {
        assert_eq!(level(&Verbosity::Quiet, true), "warn,aide=warn");
    }

    #[test]
    fn server_verbose_opens_to_debug() {
        assert_eq!(level(&Verbosity::Verbose, true), "debug,aide=warn");
    }

    #[test]
    fn cli_default_is_silent() {
        assert_eq!(level(&Verbosity::Normal, false), "off");
    }

    #[test]
    fn cli_quiet_is_silent() {
        assert_eq!(level(&Verbosity::Quiet, false), "off");
    }

    #[test]
    fn cli_verbose_restores_operational_trace() {
        assert_eq!(level(&Verbosity::Verbose, false), "info,aide=warn");
    }

    #[test]
    fn file_layer_writes_json_lines_to_dated_file() -> Result<(), Box<dyn core::error::Error>> {
        let tmp = tempfile::TempDir::new()?;
        let log_dir = tmp.path().join("logs");

        std::fs::create_dir_all(&log_dir).unwrap();

        let writer = DailyLog::new(&log_dir)?;
        let path = writer.path.clone();
        let (file_writer, guard) = tracing_appender::non_blocking(writer);

        let subscriber = tracing_subscriber::registry().with(
            tracing_subscriber::fmt::layer()
                .with_writer(file_writer)
                .json(),
        );

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!(target: "oneiros::logging::test", greeting = "hello");
        });

        drop(guard);

        let content = std::fs::read_to_string(&path)?;

        assert!(
            content.contains("hello"),
            "expected 'hello' in log output, got: {content}"
        );
        assert!(
            content.starts_with('{'),
            "expected JSON line, got: {content}"
        );

        Ok(())
    }
}
