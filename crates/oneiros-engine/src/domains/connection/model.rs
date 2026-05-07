use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Connection {
    #[builder(default)]
    pub(crate) id: ConnectionId,
    pub(crate) from_ref: Ref,
    pub(crate) to_ref: Ref,
    #[builder(into)]
    pub(crate) nature: NatureName,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Connection {
    pub(crate) fn from_token(&self) -> RefToken {
        RefToken::from(self.from_ref.clone())
    }

    pub(crate) fn to_token(&self) -> RefToken {
        RefToken::from(self.to_ref.clone())
    }
}

#[derive(Clone, Default)]
pub(crate) struct Connections(HashMap<String, Connection>);

impl Connections {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &Connection> {
        self.0.values()
    }

    pub(crate) fn get(&self, id: ConnectionId) -> Option<&Connection> {
        self.0.get(&id.to_string())
    }

    pub(crate) fn set(&mut self, connection: &Connection) -> Option<Connection> {
        self.0.insert(connection.id.to_string(), connection.clone())
    }

    pub(crate) fn remove(&mut self, connection_id: ConnectionId) -> Option<Connection> {
        self.0.remove(&connection_id.to_string())
    }
}

resource_id!(ConnectionId);
