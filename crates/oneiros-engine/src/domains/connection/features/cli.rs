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

        let prompt = match &response {
            ConnectionResponse::ConnectionCreated(wrapped) => wrapped
                .meta()
                .ref_token()
                .map(|ref_token| format!("Connection recorded: {ref_token}"))
                .unwrap_or_default(),
            ConnectionResponse::ConnectionDetails(wrapped) => format!("{:?}", wrapped.data),
            ConnectionResponse::Connections(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    let ref_token = wrapped
                        .meta()
                        .ref_token()
                        .map(|ref_token| ref_token.to_string())
                        .unwrap_or_default();
                    out.push_str(&format!(
                        "  [{}] {} -> {}\n    {}\n\n",
                        wrapped.data.nature, wrapped.data.from_ref, wrapped.data.to_ref, ref_token,
                    ));
                }
                out
            }
            ConnectionResponse::NoConnections => "No connections.".to_string(),
            ConnectionResponse::ConnectionRemoved(id) => format!("Connection {id} removed."),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
