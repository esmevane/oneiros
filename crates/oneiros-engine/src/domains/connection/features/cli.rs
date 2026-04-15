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

        let response = match self {
            ConnectionCommands::Create(creation) => connection_client.create(creation).await?,
            ConnectionCommands::Show(get) => connection_client.get(get).await?,
            ConnectionCommands::List(list) => connection_client.list(list).await?,
            ConnectionCommands::Remove(remove) => connection_client.remove(remove).await?,
        };

        Ok(ConnectionView::new(response).render().map(Into::into))
    }
}
