use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum CognitionCommands {
    Add {
        agent: String,
        texture: String,
        content: String,
    },
    Show {
        id: String,
    },
    List {
        #[arg(long)]
        agent: Option<String>,
        #[arg(long)]
        texture: Option<String>,
    },
}

impl CognitionCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match self {
            CognitionCommands::Add {
                agent,
                texture,
                content,
            } => CognitionService::add(
                context,
                &AgentName::new(&agent),
                TextureName::new(&texture),
                Content::new(content),
            )?
            .into(),
            CognitionCommands::Show { id } => {
                let id: CognitionId = id.parse()?;
                CognitionService::get(context, &id)?.into()
            }
            CognitionCommands::List { agent, texture } => CognitionService::list(
                context,
                agent.as_deref().map(AgentName::new).as_ref(),
                texture.as_deref().map(TextureName::new).as_ref(),
            )?
            .into(),
        };
        Ok(result)
    }
}
