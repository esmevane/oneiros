use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub struct SearchCommands {
    #[command(flatten)]
    pub command: SearchQuery,
}

impl SearchCommands {
    pub async fn execute(&self, context: &ProjectLog) -> Result<Rendered<Responses>, SearchError> {
        let client = context.client();
        let search_client = SearchClient::new(&client);

        let response = search_client.search(&self.command).await?;
        Ok(SearchView::new(response).render().map(Into::into))
    }
}
