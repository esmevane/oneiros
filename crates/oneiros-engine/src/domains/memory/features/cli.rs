use clap::Subcommand;

use crate::*;

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

impl MemoryCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, MemoryError> {
        let result = match self {
            MemoryCommands::Add {
                agent,
                level,
                content,
            } => MemoryService::add(
                context,
                &AgentName::new(&agent),
                LevelName::new(&level),
                Content::new(content),
            )?
            .into(),
            MemoryCommands::Show { id } => {
                let id: MemoryId = id.parse()?;
                MemoryService::get(context, &id)?.into()
            }
            MemoryCommands::List { agent } => {
                MemoryService::list(context, agent.as_deref().map(AgentName::new).as_ref())?.into()
            }
        };
        Ok(result)
    }
}
