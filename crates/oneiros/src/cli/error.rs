use thiserror::Error;

use crate::{CheckupError, ProjectCommandError, ServiceCommandError, SystemCommandError};

#[derive(Debug, Error)]
pub enum CliPreconditionError {
    #[error("No system context available.")]
    NoContext,
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("{0}")]
    Precondition(#[from] CliPreconditionError),
    #[error("Error during checkup: {0}")]
    Doctor(#[from] CheckupError),
    #[error("Problem with project: {0}")]
    Project(#[from] ProjectCommandError),
    #[error("Problem with service: {0}")]
    Service(#[from] ServiceCommandError),
    #[error("Error on host command: {0}")]
    System(#[from] SystemCommandError),
}
