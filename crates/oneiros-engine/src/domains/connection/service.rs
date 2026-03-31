use crate::*;

pub struct ConnectionService;

impl ConnectionService {
    pub async fn create(
        context: &ProjectContext,
        CreateConnection {
            from_ref,
            to_ref,
            nature,
        }: &CreateConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let from = from_ref.clone().into_inner();
        let to = to_ref.clone().into_inner();

        let connection = Connection::builder()
            .from_ref(from)
            .to_ref(to)
            .nature(nature.clone())
            .build();

        context
            .emit(ConnectionEvents::ConnectionCreated(connection.clone()))
            .await?;
        Ok(ConnectionResponse::ConnectionCreated(connection))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let connection = ConnectionRepo::new(context)
            .get(&selector.id)
            .await?
            .ok_or_else(|| ConnectionError::NotFound(selector.id))?;
        Ok(ConnectionResponse::ConnectionDetails(connection))
    }

    pub async fn list(
        context: &ProjectContext,
        ListConnections { entity }: &ListConnections,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let ref_json = entity
            .as_ref()
            .map(|token| {
                serde_json::to_string(&token.clone().into_inner())
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
        selector: &RemoveConnection,
    ) -> Result<ConnectionResponse, ConnectionError> {
        if ConnectionRepo::new(context)
            .get(&selector.id)
            .await?
            .is_none()
        {
            return Err(ConnectionError::NotFound(selector.id));
        }

        context
            .emit(ConnectionEvents::ConnectionRemoved(ConnectionRemoved {
                id: selector.id,
            }))
            .await?;
        Ok(ConnectionResponse::ConnectionRemoved(selector.id))
    }
}
