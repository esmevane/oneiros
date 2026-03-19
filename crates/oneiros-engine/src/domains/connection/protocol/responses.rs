use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionCreatedResult {
    pub id: ConnectionId,
    pub ref_token: RefToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRemovedResult {
    pub id: ConnectionId,
    pub ref_token: RefToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ConnectionResponse {
    ConnectionCreated(ConnectionCreatedResult),
    ConnectionDetails(Connection),
    Connections(Vec<Connection>),
    NoConnections,
    ConnectionRemoved(ConnectionRemovedResult),
}
