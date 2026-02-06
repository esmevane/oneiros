use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cli error: {0}")]
    Cli(#[from] CliError),
    #[error("Logging error: {0}")]
    Logging(#[from] LoggingError),
}
