use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

#[derive(Debug, Subcommand)]
pub enum BrainCommands {
    Create(CreateBrain),
    Get(GetBrain),
    List,
}

impl BrainCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
    ) -> Result<Rendered<Responses>, BrainError> {
        let response = match self {
            BrainCommands::Create(create) => {
                BrainService::create(context, create.name.clone()).await?
            }
            BrainCommands::Get(get) => BrainService::get(context, &get.name).await?,
            BrainCommands::List => BrainService::list(context).await?,
        };

        let prompt = match &response {
            BrainResponse::Created(brain) => format!("Brain '{}' created.", brain.name),
            BrainResponse::Found(brain) => format!("Brain '{}'", brain.name),
            BrainResponse::Listed(brains) => format!("{} brain(s) found.", brains.len()),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
