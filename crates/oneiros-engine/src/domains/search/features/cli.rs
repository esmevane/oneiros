use clap::Subcommand;

use crate::*;

pub struct SearchCli;

#[derive(Debug, Subcommand)]
pub enum SearchCommands {
    Search {
        query: String,
        #[arg(long)]
        agent: Option<String>,
    },
}

impl SearchCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: SearchCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            SearchCommands::Search { query, agent } => serde_json::to_string_pretty(
                &SearchService::search(ctx, &query, agent.as_deref())?,
            )?,
        };
        Ok(result)
    }
}
