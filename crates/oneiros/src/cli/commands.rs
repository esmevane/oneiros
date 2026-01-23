use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum Command {
    Doctor(crate::Doctor),
}
