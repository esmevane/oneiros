use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ConnectionResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ConnectionResponse {
    ConnectionCreated(ConnectionCreatedResponse),
    ConnectionDetails(ConnectionDetailsResponse),
    Connections(ConnectionsResponse),
    NoConnections,
    ConnectionRemoved(ConnectionRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ConnectionCreatedResponse {
        V1 => { #[serde(flatten)] pub connection: Connection }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ConnectionDetailsResponse {
        V1 => { #[serde(flatten)] pub connection: Connection }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ConnectionsResponse {
        V1 => {
            pub items: Vec<Connection>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ConnectionRemovedResponse {
        V1 => {
            pub id: ConnectionId,
        }
    }
}
