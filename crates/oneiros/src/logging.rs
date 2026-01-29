use tracing_subscriber::filter::Directive;

use crate::Error;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub(crate) fn init() -> Result<(), Error> {
    let directive: Directive = format!("{CRATE_NAME}=info").parse()?;
    let filter = tracing_subscriber::EnvFilter::from_default_env().add_directive(directive);

    tracing_subscriber::fmt().with_env_filter(filter).init();

    Ok(())
}
