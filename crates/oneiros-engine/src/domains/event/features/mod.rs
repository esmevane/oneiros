mod cli {
    use clap::Subcommand;

    use crate::*;

    /// Event inspection commands.
    #[derive(Debug, Subcommand)]
    pub enum EventCommands {
        /// List all events in the project event log.
        List,
    }

    impl EventCommands {
        pub fn execute(&self, context: &ProjectContext) -> Result<EventResponse, EventError> {
            match self {
                Self::List => Ok(EventResponse::Events(
                    context.with_db(event_repo::load_events)?,
                )),
            }
        }
    }
}

pub use cli::EventCommands;
