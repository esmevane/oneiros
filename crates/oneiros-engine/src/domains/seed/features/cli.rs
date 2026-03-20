use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SeedCommands {
    Core,
}

impl SeedCommands {
    pub fn execute(&self, ctx: &ProjectContext) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match self {
            SeedCommands::Core => SeedService::core(ctx)?.into(),
        };
        Ok(result)
    }
}
