use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub struct SearchCommands {
    pub query: String,
    #[arg(long)]
    pub agent: Option<String>,
}

impl SearchCommands {
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, SearchError> {
        let client = context.client();
        let search_client = SearchClient::new(&client);

        let agent_name = self.agent.as_deref().map(AgentName::new);
        let result = search_client
            .search(&self.query, agent_name.as_ref())
            .await?
            .into();
        Ok(result)
    }
}
