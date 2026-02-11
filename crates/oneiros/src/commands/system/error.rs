use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum SystemCommandError {
    #[error("Error during initialization: {0}")]
    Init(#[from] InitSystemError),
}
