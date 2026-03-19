use crate::*;

pub struct ConnectionService;

impl ConnectionService {
    pub fn create(
        ctx: &ProjectContext,
        from_ref: String,
        to_ref: String,
        nature: String,
        description: String,
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
            .description(description)
            .build();

        let ref_token = RefToken::new(Ref::connection(connection.id));
        ctx.emit(ConnectionEvents::ConnectionCreated(connection.clone()));
        Ok(ConnectionResponse::ConnectionCreated(
            ConnectionCreatedResult {
                id: connection.id,
                ref_token,
            },
        ))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<ConnectionResponse, ConnectionError> {
        let connection = ctx
            .with_db(|conn| ConnectionRepo::new(conn).get(id))
            .map_err(ConnectionError::Database)?
            .ok_or_else(|| ConnectionError::NotFound(id.to_string()))?;
        Ok(ConnectionResponse::ConnectionDetails(connection))
    }

    pub fn list(
        ctx: &ProjectContext,
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

        let connections = ctx
            .with_db(|conn| ConnectionRepo::new(conn).list(ref_json.as_deref()))
            .map_err(ConnectionError::Database)?;
        if connections.is_empty() {
            Ok(ConnectionResponse::NoConnections)
        } else {
            Ok(ConnectionResponse::Connections(connections))
        }
    }

    pub fn remove(ctx: &ProjectContext, id: &str) -> Result<ConnectionResponse, ConnectionError> {
        let exists = ctx
            .with_db(|conn| ConnectionRepo::new(conn).get(id))
            .map_err(ConnectionError::Database)?
            .is_some();

        if !exists {
            return Err(ConnectionError::NotFound(id.to_string()));
        }

        let id_parsed: ConnectionId = id
            .parse()
            .map_err(|e: IdParseError| ConnectionError::Database(e.into()))?;

        let ref_token = RefToken::new(Ref::connection(id_parsed));
        ctx.emit(ConnectionEvents::ConnectionRemoved(ConnectionRemoved {
            id: id_parsed,
        }));
        Ok(ConnectionResponse::ConnectionRemoved(
            ConnectionRemovedResult {
                id: id_parsed,
                ref_token,
            },
        ))
    }
}
