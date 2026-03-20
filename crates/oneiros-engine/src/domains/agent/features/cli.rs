use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    Create {
        name: AgentName,
        persona: PersonaName,
        #[arg(long, default_value = "")]
        description: Description,
        #[arg(long, default_value = "")]
        prompt: Prompt,
    },
    Show {
        name: AgentName,
    },
    List,
    Update {
        name: AgentName,
        persona: PersonaName,
        #[arg(long, default_value = "")]
        description: Description,
        #[arg(long, default_value = "")]
        prompt: Prompt,
    },
    Remove {
        name: AgentName,
    },
}

impl AgentCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, AgentError> {
        let result = match self {
            Self::Create {
                name,
                persona,
                description,
                prompt,
            } => AgentService::create(
                context,
                name.clone(),
                persona.clone(),
                description.clone(),
                prompt.clone(),
            )?
            .into(),
            Self::Show { name } => AgentService::get(context, &name)?.into(),
            Self::List => AgentService::list(context)?.into(),
            Self::Update {
                name,
                persona,
                description,
                prompt,
            } => AgentService::update(
                context,
                name.clone(),
                persona.clone(),
                description.clone(),
                prompt.clone(),
            )?
            .into(),
            Self::Remove { name } => AgentService::remove(context, &name)?.into(),
        };
        Ok(result)
    }
}
