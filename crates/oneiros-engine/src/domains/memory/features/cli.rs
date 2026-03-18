use clap::Subcommand;

use crate::*;

pub struct MemoryCli;

#[derive(Debug, Subcommand)]
pub enum MemoryCommands {
    Add {
        agent: String,
        level: String,
        content: String,
    },
    Get {
        id: String,
    },
    List {
        #[arg(long)]
        agent: Option<String>,
    },
}

impl MemoryCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: MemoryCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            MemoryCommands::Add {
                agent,
                level,
                content,
            } => serde_json::to_string_pretty(&MemoryService::add(ctx, agent, level, content)?)?,
            MemoryCommands::Get { id } => {
                serde_json::to_string_pretty(&MemoryService::get(ctx, &id)?)?
            }
            MemoryCommands::List { agent } => {
                serde_json::to_string_pretty(&MemoryService::list(ctx, agent.as_deref())?)?
            }
        };
        Ok(result)
    }
}
