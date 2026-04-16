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
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, ConnectionError> {
        let client = context.client();
        let connection_client = ConnectionClient::new(&client);

        let (response, request) = match self {
            ConnectionCommands::Create(creation) => {
                let response = connection_client.create(creation).await?;
                (
                    response,
                    ConnectionRequest::CreateConnection(creation.clone()),
                )
            }
            ConnectionCommands::Show(get) => {
                let response = connection_client.get(get).await?;
                (response, ConnectionRequest::GetConnection(get.clone()))
            }
            ConnectionCommands::List(list) => {
                let response = connection_client.list(list).await?;
                (response, ConnectionRequest::ListConnections(list.clone()))
            }
            ConnectionCommands::Remove(remove) => {
                let response = connection_client.remove(remove).await?;
                (
                    response,
                    ConnectionRequest::RemoveConnection(remove.clone()),
                )
            }
        };

        Ok(ConnectionView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
