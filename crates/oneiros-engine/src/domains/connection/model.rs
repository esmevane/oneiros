use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Connection {
    #[builder(default)]
    pub id: ConnectionId,
    pub from_ref: Ref,
    pub to_ref: Ref,
    #[builder(into)]
    pub nature: NatureName,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Clone, Default)]
pub struct Connections(HashMap<String, Connection>);

impl Connections {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn values(&self) -> impl Iterator<Item = &Connection> {
        self.0.values()
    }

    pub fn get(&self, id: ConnectionId) -> Option<&Connection> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, connection: &Connection) -> Option<Connection> {
        self.0.insert(connection.id.to_string(), connection.clone())
    }

    pub fn remove(&mut self, connection_id: ConnectionId) -> Option<Connection> {
        self.0.remove(&connection_id.to_string())
    }
}

resource_id!(ConnectionId);
