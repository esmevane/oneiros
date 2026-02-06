use clap::Subcommand;

use crate::*;

#[derive(Clone, Subcommand)]
pub(crate) enum Command {
    /// Check the health of the local oneiros host and the current project.
    Doctor(Checkup),
    /// System-level commands for the local oneiros host (init, status, etc.).
    System(System),
}
