use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum ConnectionCommands {
    Create(CreateConnection),
    Show(GetConnection),
    List(ListConnections),
    Remove(RemoveConnection),
}

impl ConnectionCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, ConnectionError> {
        let client = Client::from_config(config)?;

        let (bytes, request) = match self {
            Self::Create(creation) => (
                creation.execute_request(&client).await?,
                ConnectionRequest::CreateConnection(creation.clone()),
            ),
            Self::Show(lookup) => (
                lookup.execute_request(&client).await?,
                ConnectionRequest::GetConnection(lookup.clone()),
            ),
            Self::List(listing) => (
                listing.execute_request(&client).await?,
                ConnectionRequest::ListConnections(listing.clone()),
            ),
            Self::Remove(removal) => (
                removal.execute_request(&client).await?,
                ConnectionRequest::RemoveConnection(removal.clone()),
            ),
        };

        let response: ConnectionResponse = serde_json::from_slice(&bytes)?;
        Ok(ConnectionView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
