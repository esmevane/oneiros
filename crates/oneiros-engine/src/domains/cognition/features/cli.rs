use clap::Subcommand;

use crate::*;

pub struct CognitionCli;

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

impl CognitionCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: CognitionCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            CognitionCommands::Add {
                agent,
                texture,
                content,
            } => CognitionService::add(ctx, agent, texture, content)?.into(),
            CognitionCommands::Show { id } => CognitionService::get(ctx, &id)?.into(),
            CognitionCommands::List { agent, texture } => {
                CognitionService::list(ctx, agent.as_deref(), texture.as_deref())?.into()
            }
        };
        Ok(result)
    }
}
