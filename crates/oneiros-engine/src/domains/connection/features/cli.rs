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
            ConnectionResponse::ConnectionCreated(c) => {
                format!(
                    "Connection recorded: {}",
                    RefToken::new(Ref::connection(c.id))
                )
            }
            ConnectionResponse::ConnectionDetails(c) => format!("{c:?}"),
            ConnectionResponse::Connections(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for c in &listed.items {
                    out.push_str(&format!(
                        "  [{}] {} -> {}\n    {}\n\n",
                        c.nature,
                        c.from_ref,
                        c.to_ref,
                        RefToken::new(Ref::connection(c.id)),
                    ));
                }
                out
            }
            ConnectionResponse::NoConnections => "No connections.".to_string(),
            ConnectionResponse::ConnectionRemoved(id) => format!("Connection {id} removed."),
        };

        let envelope = match response.clone() {
            ConnectionResponse::ConnectionCreated(c) => {
                Response::new(response.into()).with_ref_token(RefToken::new(Ref::connection(c.id)))
            }
            otherwise => Response::new(otherwise.into()),
        };

        Ok(Rendered::new(envelope, prompt, String::new()))
    }
}
