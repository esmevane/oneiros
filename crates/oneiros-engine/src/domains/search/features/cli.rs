use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub(crate) struct SearchCommands {
    #[command(flatten)]
    pub(crate) command: SearchQuery,
}

impl SearchCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, SearchError> {
        let client = Client::from_config(config)?;
        let bytes = self.command.execute_request(&client).await?;
        let response: SearchResponse = serde_json::from_slice(&bytes)?;
        Ok(SearchView::new(response).render().map(Into::into))
    }
}
