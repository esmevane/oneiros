use chrono::Utc;
use uuid::Uuid;

use crate::contexts::ProjectContext;

use super::errors::ConnectionError;
use super::model::Connection;
use super::repo::ConnectionRepo;
use super::responses::ConnectionResponse;

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
            id: Uuid::now_v7().to_string(),
            from_entity,
            to_entity,
            nature,
            description,
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit("connection-created", &connection);
        Ok(ConnectionResponse::Created(connection))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<ConnectionResponse, ConnectionError> {
        let connection = ctx
            .with_db(|conn| ConnectionRepo::new(conn).get(id))
            .map_err(ConnectionError::Database)?
            .ok_or_else(|| ConnectionError::NotFound(id.to_string()))?;
        Ok(ConnectionResponse::Found(connection))
    }

    pub fn list(
        ctx: &ProjectContext,
        entity: Option<&str>,
    ) -> Result<ConnectionResponse, ConnectionError> {
        let connections = ctx
            .with_db(|conn| ConnectionRepo::new(conn).list(entity))
            .map_err(ConnectionError::Database)?;
        Ok(ConnectionResponse::Listed(connections))
    }

    pub fn remove(ctx: &ProjectContext, id: &str) -> Result<ConnectionResponse, ConnectionError> {
        // Confirm existence before emitting removal.
        let exists = ctx
            .with_db(|conn| ConnectionRepo::new(conn).get(id))
            .map_err(ConnectionError::Database)?
            .is_some();

        if !exists {
            return Err(ConnectionError::NotFound(id.to_string()));
        }

        ctx.emit("connection-removed", &serde_json::json!({ "id": id }));
        Ok(ConnectionResponse::Removed)
    }
}
