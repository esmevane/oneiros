use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ContinuityCommands {
    Wake {
        agent: AgentName,
    },
    Dream {
        agent: AgentName,
    },
    Introspect {
        agent: AgentName,
    },
    Reflect {
        agent: AgentName,
    },
    Sense {
        agent: AgentName,
        content: Content,
    },
    Sleep {
        agent: AgentName,
    },
    Guidebook {
        agent: AgentName,
    },
    Emerge {
        name: AgentName,
        persona: PersonaName,
        #[arg(long, default_value = "")]
        description: Description,
    },
    Recede {
        agent: AgentName,
    },
    Status {
        agent: AgentName,
    },
}

impl ContinuityCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, ContinuityError> {
        let client = context.client();
        let continuity_client = ContinuityClient::new(&client);

        let result = match self {
            ContinuityCommands::Wake { agent } => {
                ContinuityPresenter::new(continuity_client.wake(agent).await?).render()
            }
            ContinuityCommands::Dream { agent } => {
                ContinuityPresenter::new(continuity_client.dream(agent).await?).render()
            }
            ContinuityCommands::Introspect { agent } => {
                ContinuityPresenter::new(continuity_client.introspect(agent).await?).render()
            }
            ContinuityCommands::Reflect { agent } => {
                ContinuityPresenter::new(continuity_client.reflect(agent).await?).render()
            }
            ContinuityCommands::Sense { agent, content } => {
                ContinuityPresenter::new(continuity_client.sense(agent, content.clone()).await?)
                    .render()
            }
            ContinuityCommands::Sleep { agent } => {
                ContinuityPresenter::new(continuity_client.sleep(agent).await?).render()
            }
            ContinuityCommands::Guidebook { agent } => {
                ContinuityPresenter::new(continuity_client.guidebook(agent).await?).render()
            }
            ContinuityCommands::Emerge {
                name,
                persona,
                description,
            } => ContinuityPresenter::new(
                continuity_client
                    .emerge(name.clone(), persona.clone(), description.clone())
                    .await?,
            )
            .render(),
            ContinuityCommands::Recede { agent } => {
                ContinuityPresenter::new(continuity_client.recede(agent).await?).render()
            }
            ContinuityCommands::Status { agent } => {
                ContinuityPresenter::new(continuity_client.status(agent).await?).render()
            }
        };
        Ok(result)
    }
}
