use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ConnectionEvents {
    ConnectionCreated(Connection),
    ConnectionRemoved { id: ConnectionId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    pub nature: NatureName,
    pub from_ref: Ref,
    pub to_ref: Ref,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ConnectionRequests {
    CreateConnection(CreateConnectionRequest),
    RemoveConnection { id: ConnectionId },
    GetConnection { id: ConnectionId },
    ListConnections,
}
