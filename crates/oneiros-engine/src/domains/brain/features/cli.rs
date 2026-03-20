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
    pub fn execute(context: &SystemContext, cmd: BrainCommands) -> Result<Responses, BrainError> {
        let result = match cmd {
            BrainCommands::Create { name } => {
                BrainService::create(context, BrainName::new(name))?.into()
            }
            BrainCommands::Get { name } => {
                BrainService::get(context, &BrainName::new(name))?.into()
            }
            BrainCommands::List => BrainService::list(context)?.into(),
        };
        Ok(result)
    }
}
