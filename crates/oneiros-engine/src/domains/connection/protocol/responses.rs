use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ConnectionResponse {
    ConnectionCreated(Response<Connection>),
    ConnectionDetails(Response<Connection>),
    Connections(Listed<Response<Connection>>),
    NoConnections,
    ConnectionRemoved(ConnectionId),
}
