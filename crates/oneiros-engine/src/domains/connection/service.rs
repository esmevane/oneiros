use chrono::Utc;

use crate::*;

pub struct ConnectionService;

impl ConnectionService {
    pub fn create(
        ctx: &ProjectContext,
        from_entity: String,
        to_entity: String,
        nature: String,
        description: String,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let connection = Connection {
            id: ConnectionId::new(),
            from_entity,
            to_entity,
            nature,
            description,
            created_at: Utc::now().to_rfc3339(),
        };

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
        entity: Option<&str>,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let connections = ctx
            .with_db(|conn| ConnectionRepo::new(conn).list(entity))
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
