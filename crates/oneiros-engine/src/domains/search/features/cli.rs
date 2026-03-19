use clap::Args;

use crate::*;

pub struct SearchCli;

#[derive(Debug, Args)]
pub struct SearchCommands {
    pub query: String,
    #[arg(long)]
    pub agent: Option<String>,
}

impl SearchCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: SearchCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = SearchService::search(ctx, &cmd.query, cmd.agent.as_deref())?.into();
        Ok(result)
    }
}
