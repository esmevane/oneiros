#[derive(Debug, thiserror::Error)]
#[error("Logging error: {0}")]
pub struct LoggingError(#[from] tracing_subscriber::filter::ParseError);

pub(crate) fn init() -> Result<(), LoggingError> {
    let filter = tracing_subscriber::EnvFilter::from_default_env();

    tracing_subscriber::fmt().with_env_filter(filter).init();

    Ok(())
}
