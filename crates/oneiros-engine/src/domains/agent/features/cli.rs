use clap::Subcommand;

use crate::*;

pub struct AgentCli;

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    Create {
        name: String,
        persona: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long, default_value = "")]
        prompt: String,
    },
    Show {
        name: String,
    },
    List,
    Update {
        name: String,
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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            AgentCommands::Create {
                name,
                persona,
                description,
                prompt,
            } => AgentService::create(ctx, name, persona, description, prompt)?.into(),
            AgentCommands::Show { name } => AgentService::get(ctx, &name)?.into(),
            AgentCommands::List => AgentService::list(ctx)?.into(),
            AgentCommands::Update {
                name,
                persona,
                description,
                prompt,
            } => AgentService::update(ctx, name, persona, description, prompt)?.into(),
            AgentCommands::Remove { name } => AgentService::remove(ctx, &name)?.into(),
        };
        Ok(result)
    }
}
