use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SeedCommands {
    Core,
}

impl SeedCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, SeedError> {
        let response = match self {
            SeedCommands::Core => SeedService::core(context).await?,
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
