use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ConnectionResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum ConnectionResponse {
    ConnectionCreated(ConnectionCreatedResponse),
    ConnectionDetails(ConnectionDetailsResponse),
    Connections(ConnectionsResponse),
    NoConnections,
    ConnectionRemoved(ConnectionRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ConnectionCreatedResponse {
        V1 => { #[serde(flatten)] pub(crate) connection: Connection }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ConnectionDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) connection: Connection }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ConnectionsResponse {
        V1 => {
            pub(crate) items: Vec<Connection>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ConnectionRemovedResponse {
        V1 => {
            pub(crate) id: ConnectionId,
        }
    }
}
