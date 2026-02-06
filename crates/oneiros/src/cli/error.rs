use thiserror::Error;

use crate::{CheckupError, SystemCommandError};

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Error during checkup: {0}")]
    Doctor(#[from] CheckupError),
    #[error("Error on host command: {0}")]
    System(#[from] SystemCommandError),
}
