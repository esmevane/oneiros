use clap::Subcommand;

use crate::*;

/// CLI subcommands for the agent domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    Create(CreateAgent),
    Show(GetAgent),
    List(ListAgents),
    Update(UpdateAgent),
    Remove(RemoveAgent),
}

impl AgentCommands {
    pub async fn execute(&self, context: &ProjectLog) -> Result<Rendered<Responses>, AgentError> {
        let client = context.client();
        let agent_client = AgentClient::new(&client);

        let response = match self {
            Self::Create(creation) => agent_client.create(creation).await?,
            Self::Show(lookup) => agent_client.get(lookup).await?,
            Self::List(listing) => agent_client.list(listing).await?,
            Self::Update(update) => agent_client.update(update).await?,
            Self::Remove(removal) => agent_client.remove(removal).await?,
        };

        Ok(AgentView::new(response).render().map(Into::into))
    }
}
