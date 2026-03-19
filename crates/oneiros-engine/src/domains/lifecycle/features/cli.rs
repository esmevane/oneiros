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
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            LifecycleCommands::Wake { agent } => {
                serde_json::to_string_pretty(&LifecycleService::wake(ctx, &agent)?)?
            }
            LifecycleCommands::Dream { agent } => {
                serde_json::to_string_pretty(&LifecycleService::dream(ctx, &agent)?)?
            }
            LifecycleCommands::Introspect { agent } => {
                serde_json::to_string_pretty(&LifecycleService::introspect(ctx, &agent)?)?
            }
            LifecycleCommands::Reflect { agent } => {
                serde_json::to_string_pretty(&LifecycleService::reflect(ctx, &agent)?)?
            }
            LifecycleCommands::Sense { agent, content } => {
                serde_json::to_string_pretty(&LifecycleService::sense(ctx, &agent, &content)?)?
            }
            LifecycleCommands::Sleep { agent } => {
                serde_json::to_string_pretty(&LifecycleService::sleep(ctx, &agent)?)?
            }
            LifecycleCommands::Guidebook { agent } => {
                serde_json::to_string_pretty(&LifecycleService::guidebook(ctx, &agent)?)?
            }
        };
        Ok(result)
    }
}
