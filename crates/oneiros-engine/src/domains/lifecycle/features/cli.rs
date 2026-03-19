use clap::Subcommand;

use crate::*;

pub struct LifecycleCli;

#[derive(Debug, Subcommand)]
pub enum LifecycleCommands {
    Wake { agent: String },
    Dream { agent: String },
    Introspect { agent: String },
    Reflect { agent: String },
    Sense { agent: String, content: String },
    Sleep { agent: String },
    Guidebook { agent: String },
}

impl LifecycleCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: LifecycleCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            LifecycleCommands::Wake { agent } => LifecycleService::wake(ctx, &agent)?.into(),
            LifecycleCommands::Dream { agent } => LifecycleService::dream(ctx, &agent)?.into(),
            LifecycleCommands::Introspect { agent } => {
                LifecycleService::introspect(ctx, &agent)?.into()
            }
            LifecycleCommands::Reflect { agent } => LifecycleService::reflect(ctx, &agent)?.into(),
            LifecycleCommands::Sense { agent, content } => {
                LifecycleService::sense(ctx, &agent, &content)?.into()
            }
            LifecycleCommands::Sleep { agent } => LifecycleService::sleep(ctx, &agent)?.into(),
            LifecycleCommands::Guidebook { agent } => {
                LifecycleService::guidebook(ctx, &agent)?.into()
            }
        };
        Ok(result)
    }
}
