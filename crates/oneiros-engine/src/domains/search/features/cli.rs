use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub struct SearchCommands {
    pub query: String,
    #[arg(long)]
    pub agent: Option<String>,
}

impl SearchCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, SearchError> {
        let agent_name = self.agent.as_deref().map(AgentName::new);
        let result = SearchService::search(context, &self.query, agent_name.as_ref())?.into();
        Ok(result)
    }
}
