mod cli {
    use clap::Subcommand;

    /// Event inspection commands.
    #[derive(Debug, Subcommand)]
    pub enum EventCommands {}
}

pub use cli::EventCommands;
