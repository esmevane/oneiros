use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub(crate) struct SearchCommands {
    #[command(flatten)]
    pub(crate) query: SearchQuery,
}

impl SearchCommands {
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, SearchError> {
        
        let search_client = SearchClient::new(client);

        let response = search_client.search(&self.query).await?;
        Ok(SearchView::new(response).render())
    }
}
