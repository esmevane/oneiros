use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub(crate) struct Connection {
    #[builder(default)]
    pub(crate) id: ConnectionId,
    #[loro(json)]
    pub(crate) from_ref: Ref,
    #[loro(json)]
    pub(crate) to_ref: Ref,
    #[builder(into)]
    pub(crate) nature: NatureName,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "connections")]
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
