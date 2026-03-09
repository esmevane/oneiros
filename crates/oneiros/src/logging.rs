#[derive(Debug, thiserror::Error)]
#[error("Logging error: {0}")]
pub struct LoggingError(#[from] tracing_subscriber::filter::ParseError);

pub(crate) fn init(config: &super::LogConfig) -> Result<(), LoggingError> {
    // The verbosity flag sets the default level. RUST_LOG directives layer
    // on top, adding granularity without overriding the base.
    //
    //   oneiros -vv                            → base at debug
    //   RUST_LOG=oneiros_db=trace oneiros -vv  → debug + trace for oneiros_db
    //   RUST_LOG=warn oneiros -vvv             → RUST_LOG overrides to warn
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(config.verbosity.tracing_level_filter().into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();

    Ok(())
}
