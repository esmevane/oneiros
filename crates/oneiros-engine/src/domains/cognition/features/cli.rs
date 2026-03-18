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
    Get {
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
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            CognitionCommands::Add {
                agent,
                texture,
                content,
            } => serde_json::to_string_pretty(&CognitionService::add(
                ctx, agent, texture, content,
            )?)?,
            CognitionCommands::Get { id } => {
                serde_json::to_string_pretty(&CognitionService::get(ctx, &id)?)?
            }
            CognitionCommands::List { agent, texture } => serde_json::to_string_pretty(
                &CognitionService::list(ctx, agent.as_deref(), texture.as_deref())?,
            )?,
        };
        Ok(result)
    }
}
