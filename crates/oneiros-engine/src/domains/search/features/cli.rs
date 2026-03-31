use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub struct SearchCommands {
    #[command(flatten)]
    pub query: SearchQuery,
}

impl SearchCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, SearchError> {
        let client = context.client();
        let search_client = SearchClient::new(&client);

        let response = search_client.search(&self.query).await?;
        Ok(SearchPresenter::new(response).render())
    }
}
