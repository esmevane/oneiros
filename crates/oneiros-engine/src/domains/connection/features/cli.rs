use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ConnectionCommands {
    Create(CreateConnection),
    Show(GetConnection),
    List(ListConnections),
    Remove(RemoveConnection),
}

impl ConnectionCommands {
    pub async fn execute(
        &self,
        context: &ProjectLog,
    ) -> Result<Rendered<Responses>, ConnectionError> {
        let client = context.client();
        let connection_client = ConnectionClient::new(&client);

        let (response, request) = match self {
            Self::Create(creation) => (
                connection_client.create(creation).await?,
                ConnectionRequest::CreateConnection(creation.clone()),
            ),
            Self::Show(lookup) => (
                connection_client.get(lookup).await?,
                ConnectionRequest::GetConnection(lookup.clone()),
            ),
            Self::List(listing) => (
                connection_client.list(listing).await?,
                ConnectionRequest::ListConnections(listing.clone()),
            ),
            Self::Remove(removal) => (
                connection_client.remove(removal).await?,
                ConnectionRequest::RemoveConnection(removal.clone()),
            ),
        };

        Ok(ConnectionView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
