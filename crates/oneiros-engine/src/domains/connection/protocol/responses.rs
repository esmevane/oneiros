use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ConnectionResponse {
    ConnectionCreated(Connection),
    ConnectionDetails(Connection),
    Connections(Vec<Connection>),
    NoConnections,
    ConnectionRemoved(ConnectionId),
}
