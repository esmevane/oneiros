use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

pub struct BrainCli;

#[derive(Debug, Subcommand)]
pub enum BrainCommands {
    Create { name: String },
    Get { name: String },
    List,
}

impl BrainCli {
    pub fn execute(
        ctx: &SystemContext,
        cmd: BrainCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            BrainCommands::Create { name } => BrainService::create(ctx, name)?.into(),
            BrainCommands::Get { name } => BrainService::get(ctx, &name)?.into(),
            BrainCommands::List => BrainService::list(ctx)?.into(),
        };
        Ok(result)
    }
}
