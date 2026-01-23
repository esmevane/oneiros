use tracing_subscriber::filter::Directive;

use crate::Error;

pub(crate) fn init(config: impl super::Cli) -> Result<(), Error> {
    let package_name = config.project_name();
    let directive: Directive = format!("{package_name}=info").parse()?;
    let filter = tracing_subscriber::EnvFilter::from_default_env().add_directive(directive);

    tracing_subscriber::fmt().with_env_filter(filter).init();

    Ok(())
}
