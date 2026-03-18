use clap::Subcommand;

use crate::*;

pub struct SeedCli;

#[derive(Debug, Subcommand)]
pub enum SeedCommands {
    Core,
}

impl SeedCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: SeedCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match cmd {
            SeedCommands::Core => {
                let result = SeedService::core(ctx)?;
                Ok(serde_json::to_string_pretty(&result)?)
            }
        }
    }
}
