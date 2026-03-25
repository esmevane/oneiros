use crate::*;

pub struct ConnectionService;

impl ConnectionService {
    pub async fn create(
        context: &ProjectContext,
        from_ref: String,
        to_ref: String,
        nature: String,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let from: Ref = from_ref
            .parse::<RefToken>()
            .map_err(|e| ConnectionError::InvalidRef(e.to_string()))?
            .into_inner();
        let to: Ref = to_ref
            .parse::<RefToken>()
            .map_err(|e| ConnectionError::InvalidRef(e.to_string()))?
            .into_inner();

        let connection = Connection::builder()
            .from_ref(from)
            .to_ref(to)
            .nature(nature)
            .build();

        context
            .emit(ConnectionEvents::ConnectionCreated(connection.clone()))
            .await?;
        Ok(ConnectionResponse::ConnectionCreated(connection))
    }

    pub async fn get(
        context: &ProjectContext,
        id: &ConnectionId,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let connection = ConnectionRepo::new(context)
            .get(id)
            .await?
            .ok_or_else(|| ConnectionError::NotFound(*id))?;
        Ok(ConnectionResponse::ConnectionDetails(connection))
    }

    pub async fn list(
        context: &ProjectContext,
        entity_ref: Option<&str>,
    ) -> Result<ConnectionResponse, ConnectionError> {
        // If an entity ref is provided, parse it and JSON-encode for the DB query
        let ref_json = entity_ref
            .map(|s| {
                let token: RefToken = s
                    .parse()
                    .map_err(|e: RefError| ConnectionError::InvalidRef(e.to_string()))?;
                serde_json::to_string(&token.into_inner())
                    .map_err(|e| ConnectionError::Event(e.into()))
            })
            .transpose()?;

        let connections = ConnectionRepo::new(context)
            .list(ref_json.as_deref())
            .await?;
        if connections.is_empty() {
            Ok(ConnectionResponse::NoConnections)
        } else {
            Ok(ConnectionResponse::Connections(connections))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        id: &ConnectionId,
    ) -> Result<ConnectionResponse, ConnectionError> {
        if ConnectionRepo::new(context).get(id).await?.is_none() {
            return Err(ConnectionError::NotFound(*id));
        }

        context
            .emit(ConnectionEvents::ConnectionRemoved(ConnectionRemoved {
                id: *id,
            }))
            .await?;
        Ok(ConnectionResponse::ConnectionRemoved(*id))
    }
}
