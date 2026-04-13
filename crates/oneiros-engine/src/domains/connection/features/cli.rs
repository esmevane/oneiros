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

        let prompt = match &response {
            ConnectionResponse::ConnectionCreated(wrapped) => ConnectionView::recorded(wrapped),
            ConnectionResponse::ConnectionDetails(wrapped) => {
                ConnectionView::detail(&wrapped.data).to_string()
            }
            ConnectionResponse::Connections(listed) => {
                let table = ConnectionView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            ConnectionResponse::NoConnections => format!("{}", "No connections.".muted()),
            ConnectionResponse::ConnectionRemoved(id) => ConnectionView::removed(id).to_string(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
