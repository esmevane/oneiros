use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

#[derive(Debug, Subcommand)]
pub enum BrainCommands {
    Create { name: String },
    Get { name: String },
    List,
}

impl BrainCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
    ) -> Result<Rendered<Responses>, BrainError> {
        let response = match self {
            BrainCommands::Create { name } => {
                BrainService::create(context, BrainName::new(name)).await?
            }
            BrainCommands::Get { name } => BrainService::get(context, &BrainName::new(name))?,
            BrainCommands::List => BrainService::list(context)?,
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
