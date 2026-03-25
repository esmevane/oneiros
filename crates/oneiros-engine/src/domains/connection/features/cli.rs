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

        match self {
            ConnectionCommands::Create(creation) => {
                let from = creation.from_ref.clone().into_inner();
                let to = creation.to_ref.clone().into_inner();
                let response = connection_client
                    .create(from, to, creation.nature.clone())
                    .await?;
                let ref_token = match &response {
                    ConnectionResponse::ConnectionCreated(c) => {
                        Some(RefToken::new(Ref::connection(c.id)))
                    }
                    _ => None,
                };

                let prompt = ref_token
                    .as_ref()
                    .map(|rt| format!("Connection recorded: {rt}"))
                    .unwrap_or_default();

                let mut envelope = Response::new(response.into());
                if let Some(rt) = ref_token {
                    envelope = envelope.with_ref_token(rt);
                }

                Ok(Rendered::new(envelope, prompt, String::new()))
            }
            ConnectionCommands::Show(get) => {
                let response = connection_client.get(&get.id).await?;
                let prompt = match &response {
                    ConnectionResponse::ConnectionCreated(c)
                    | ConnectionResponse::ConnectionDetails(c) => {
                        format!("{c:?}")
                    }
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
                ))
            }
            ConnectionCommands::List(list) => {
                let token = list.entity.clone();
                let response = connection_client
                    .list(token.map(|entity| entity.clone().into_inner()).as_ref())
                    .await?;
                let prompt = match &response {
                    ConnectionResponse::Connections(list) => format!("{} connections.", list.len()),
                    ConnectionResponse::NoConnections => "No connections.".to_string(),
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
                ))
            }
            ConnectionCommands::Remove(remove) => {
                let response = connection_client.remove(&remove.id).await?;
                let prompt = match &response {
                    ConnectionResponse::ConnectionRemoved(id) => {
                        format!("Connection {id} removed.")
                    }
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
                ))
            }
        }
    }
}
