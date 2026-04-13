mod cli {
    use clap::Subcommand;

    /// Event inspection commands.
    #[derive(Debug, Subcommand)]
    pub(crate) enum EventCommands {}
}

pub(crate) use cli::*;
