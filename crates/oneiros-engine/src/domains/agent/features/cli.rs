use clap::Subcommand;

use crate::*;

pub struct AgentCli;

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    Create {
        name: String,
        #[arg(long)]
        persona: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long, default_value = "")]
        prompt: String,
    },
    Get {
        name: String,
    },
    List,
    Update {
        name: String,
        #[arg(long)]
        persona: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long, default_value = "")]
        prompt: String,
    },
    Remove {
        name: String,
    },
}

impl AgentCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: AgentCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            AgentCommands::Create {
                name,
                persona,
                description,
                prompt,
            } => serde_json::to_string_pretty(&AgentService::create(
                ctx,
                name,
                persona,
                description,
                prompt,
            )?)?,
            AgentCommands::Get { name } => {
                serde_json::to_string_pretty(&AgentService::get(ctx, &name)?)?
            }
            AgentCommands::List => serde_json::to_string_pretty(&AgentService::list(ctx)?)?,
            AgentCommands::Update {
                name,
                persona,
                description,
                prompt,
            } => serde_json::to_string_pretty(&AgentService::update(
                ctx,
                name,
                persona,
                description,
                prompt,
            )?)?,
            AgentCommands::Remove { name } => {
                serde_json::to_string_pretty(&AgentService::remove(ctx, &name)?)?
            }
        };
        Ok(result)
    }
}
