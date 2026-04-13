use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum AgentCommands {
    Create(CreateAgent),
    Show(GetAgent),
    List(ListAgents),
    Update(UpdateAgent),
    Remove(RemoveAgent),
}

impl AgentCommands {
    pub(crate) async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, AgentError> {
        let client = context.client();
        let agent_client = AgentClient::new(&client);

        let response = match self {
            Self::Create(creation) => agent_client.create(creation).await?,
            Self::Show(get) => agent_client.get(&get.name).await?,
            Self::List(listing) => agent_client.list(listing).await?,
            Self::Update(update) => agent_client.update(update).await?,
            Self::Remove(removal) => agent_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            AgentResponse::AgentCreated(name) => AgentView::confirmed("created", name).to_string(),
            AgentResponse::AgentDetails(wrapped) => AgentView::detail(&wrapped.data).to_string(),
            AgentResponse::Agents(listed) => {
                let table = AgentView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            AgentResponse::NoAgents => format!("{}", "No agents configured.".muted()),
            AgentResponse::AgentUpdated(name) => AgentView::confirmed("updated", name).to_string(),
            AgentResponse::AgentRemoved(name) => AgentView::confirmed("removed", name).to_string(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
