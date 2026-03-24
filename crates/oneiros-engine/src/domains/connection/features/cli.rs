use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ConnectionCommands {
    Create {
        nature: String,
        from_ref: String,
        to_ref: String,
    },
    Show {
        id: String,
    },
    List {
        #[arg(long)]
        nature: Option<String>,
        #[arg(long)]
        entity_ref: Option<String>,
    },
    Remove {
        id: String,
    },
}

impl ConnectionCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, ConnectionError> {
        let client = context.client();
        let connection_client = ConnectionClient::new(&client);

        match self {
            ConnectionCommands::Create {
                nature,
                from_ref,
                to_ref,
            } => {
                let from: Ref = from_ref
                    .parse::<RefToken>()
                    .map_err(|e| ConnectionError::InvalidRef(e.to_string()))?
                    .into_inner();
                let to: Ref = to_ref
                    .parse::<RefToken>()
                    .map_err(|e| ConnectionError::InvalidRef(e.to_string()))?
                    .into_inner();
                let response = connection_client
                    .create(from, to, NatureName::new(nature))
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
            ConnectionCommands::Show { id } => {
                let id: ConnectionId = id.parse()?;
                let response = connection_client.get(&id).await?;
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
            ConnectionCommands::List { entity_ref, .. } => {
                let entity = entity_ref
                    .as_deref()
                    .map(|s| {
                        s.parse::<RefToken>()
                            .map_err(|e| ConnectionError::InvalidRef(e.to_string()))
                            .map(RefToken::into_inner)
                    })
                    .transpose()?;
                let response = connection_client.list(entity.as_ref()).await?;
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
            ConnectionCommands::Remove { id } => {
                let id: ConnectionId = id.parse()?;
                let response = connection_client.remove(&id).await?;
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
