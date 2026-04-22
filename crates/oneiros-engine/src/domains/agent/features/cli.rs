use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    Create(CreateAgent),
    Show(GetAgent),
    List(ListAgents),
    Update(UpdateAgent),
    Remove(RemoveAgent),
}

impl AgentCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, AgentError> {
        let client = context.client();
        let agent_client = AgentClient::new(&client);

        let response = match self {
            Self::Create(creation) => agent_client.create(creation).await?,
            Self::Show(get) => agent_client.get(get).await?,
            Self::List(listing) => agent_client.list(listing).await?,
            Self::Update(update) => agent_client.update(update).await?,
            Self::Remove(removal) => agent_client.remove(&removal.name).await?,
        };

        Ok(AgentView::new(response).render().map(Into::into))
    }
}
