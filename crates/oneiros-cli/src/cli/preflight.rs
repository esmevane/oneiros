use clap::Parser;

use super::LogConfig;

/// Lenient CLI parse that extracts logging config before full validation.
///
/// This runs with `ignore_errors = true` so it succeeds even with unknown
/// or malformed flags. The result is used to initialize tracing before the
/// full CLI parse, so bootstrap errors are visible at the right log level.
#[derive(Parser, Default)]
#[command(ignore_errors = true)]
pub(crate) struct Preflight {
    #[command(flatten)]
    pub(crate) log: LogConfig,
}

impl Preflight {
    pub(crate) fn preflight_parse() -> Self {
        Self::try_parse().unwrap_or_default()
    }
}
