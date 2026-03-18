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
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            BrainCommands::Create { name } => {
                serde_json::to_string_pretty(&BrainService::create(ctx, name)?)?
            }
            BrainCommands::Get { name } => {
                serde_json::to_string_pretty(&BrainService::get(ctx, &name)?)?
            }
            BrainCommands::List => serde_json::to_string_pretty(&BrainService::list(ctx)?)?,
        };
        Ok(result)
    }
}
