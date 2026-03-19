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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            SeedCommands::Core => SeedService::core(ctx)?.into(),
        };
        Ok(result)
    }
}
