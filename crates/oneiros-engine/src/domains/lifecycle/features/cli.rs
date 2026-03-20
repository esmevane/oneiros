use clap::Subcommand;

use crate::*;

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

impl LifecycleCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, LifecycleError> {
        let result = match self {
            LifecycleCommands::Wake { agent } => {
                LifecycleService::wake(context, &AgentName::new(&agent))?.into()
            }
            LifecycleCommands::Dream { agent } => {
                LifecycleService::dream(context, &AgentName::new(&agent))?.into()
            }
            LifecycleCommands::Introspect { agent } => {
                LifecycleService::introspect(context, &AgentName::new(&agent))?.into()
            }
            LifecycleCommands::Reflect { agent } => {
                LifecycleService::reflect(context, &AgentName::new(&agent))?.into()
            }
            LifecycleCommands::Sense { agent, content } => {
                LifecycleService::sense(context, &AgentName::new(&agent), &Content::new(&content))?
                    .into()
            }
            LifecycleCommands::Sleep { agent } => {
                LifecycleService::sleep(context, &AgentName::new(&agent))?.into()
            }
            LifecycleCommands::Guidebook { agent } => {
                LifecycleService::guidebook(context, &AgentName::new(&agent))?.into()
            }
        };
        Ok(result)
    }
}
