use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub(crate) struct SearchCommands {
    #[command(flatten)]
    pub(crate) command: SearchQuery,
}

impl SearchCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, SearchError> {
        let client = Client::new(config.base_url());
        let search_client = SearchClient::new(&client);

        let response = search_client.search(&self.command).await?;
        Ok(SearchView::new(response).render().map(Into::into))
    }
}
