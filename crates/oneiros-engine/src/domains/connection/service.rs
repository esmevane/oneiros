use crate::*;

pub struct ConnectionService;

impl ConnectionService {
    pub fn create(
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

        context.emit(ConnectionEvents::ConnectionCreated(connection.clone()));
        Ok(ConnectionResponse::ConnectionCreated(connection))
    }

    pub fn get(
        context: &ProjectContext,
        id: &ConnectionId,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let connection = context
            .with_db(|conn| ConnectionRepo::new(conn).get(id))?
            .ok_or_else(|| ConnectionError::NotFound(*id))?;
        Ok(ConnectionResponse::ConnectionDetails(connection))
    }

    pub fn list(
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
                    .map_err(|e| ConnectionError::Database(e.into()))
            })
            .transpose()?;

        let connections = context
            .with_db(|conn| ConnectionRepo::new(conn).list(ref_json.as_deref()))
            .map_err(ConnectionError::Database)?;
        if connections.is_empty() {
            Ok(ConnectionResponse::NoConnections)
        } else {
            Ok(ConnectionResponse::Connections(connections))
        }
    }

    pub fn remove(
        context: &ProjectContext,
        id: &ConnectionId,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let exists = context
            .with_db(|conn| ConnectionRepo::new(conn).get(id))
            .map_err(ConnectionError::Database)?
            .is_some();

        if !exists {
            return Err(ConnectionError::NotFound(*id));
        }

        context.emit(ConnectionEvents::ConnectionRemoved(ConnectionRemoved {
            id: *id,
        }));
        Ok(ConnectionResponse::ConnectionRemoved(*id))
    }
}
