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
    Show {
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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            MemoryCommands::Add {
                agent,
                level,
                content,
            } => MemoryService::add(ctx, agent, level, content)?.into(),
            MemoryCommands::Show { id } => MemoryService::get(ctx, &id)?.into(),
            MemoryCommands::List { agent } => MemoryService::list(ctx, agent.as_deref())?.into(),
        };
        Ok(result)
    }
}
