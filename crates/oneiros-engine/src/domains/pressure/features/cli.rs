use clap::Subcommand;

use crate::*;

pub struct PressureCli;

#[derive(Debug, Subcommand)]
pub enum PressureCommands {
    Get { agent: String },
    List,
}

impl PressureCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: PressureCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            PressureCommands::Get { agent } => {
                serde_json::to_string_pretty(&PressureService::get(ctx, &agent)?)?
            }
            PressureCommands::List => serde_json::to_string_pretty(&PressureService::list(ctx)?)?,
        };
        Ok(result)
    }
}
