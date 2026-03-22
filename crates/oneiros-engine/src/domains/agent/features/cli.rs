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
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, AgentError> {
        let client = context.client();
        let agent_client = AgentClient::new(&client);

        let result = match self {
            Self::Create {
                name,
                persona,
                description,
                prompt,
            } => agent_client
                .create(
                    name.clone(),
                    persona.clone(),
                    description.clone(),
                    prompt.clone(),
                )
                .await?
                .into(),
            Self::Show { name } => agent_client.get(name).await?.into(),
            Self::List => agent_client.list().await?.into(),
            Self::Update {
                name,
                persona,
                description,
                prompt,
            } => agent_client
                .update(name, persona.clone(), description.clone(), prompt.clone())
                .await?
                .into(),
            Self::Remove { name } => agent_client.remove(name).await?.into(),
        };

        Ok(result)
    }
}
