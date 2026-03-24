use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SeedCommands {
    Core,
}

impl SeedCommands {
    pub fn execute(
        &self,
        ctx: &ProjectContext,
    ) -> Result<Rendered<Responses>, Box<dyn std::error::Error>> {
        let response = match self {
            SeedCommands::Core => SeedService::core(ctx)?,
        };

        let prompt = match &response {
            SeedResponse::SeedComplete => "Core seed complete.".to_string(),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
