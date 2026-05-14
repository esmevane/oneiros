use clap::Subcommand;

use crate::*;

/// CLI subcommands for the agent domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
#[derive(Debug, Subcommand)]
pub(crate) enum AgentCommands {
    Create(CreateAgent),
    Show(GetAgent),
    List(ListAgents),
    Update(UpdateAgent),
    Remove(RemoveAgent),
}

impl AgentCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, AgentError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Create(creation) => creation.execute_request(&client).await?,
            Self::Show(lookup) => lookup.execute_request(&client).await?,
            Self::List(listing) => listing.execute_request(&client).await?,
            Self::Update(update) => update.execute_request(&client).await?,
            Self::Remove(removal) => removal.execute_request(&client).await?,
        };

        let response: AgentResponse = serde_json::from_slice(&bytes)?;
        Ok(AgentView::new(response).render().map(Into::into))
    }
}
