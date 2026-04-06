use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Connection {
    #[builder(default)]
    pub id: ConnectionId,
    #[loro(json)]
    pub from_ref: Ref,
    #[loro(json)]
    pub to_ref: Ref,
    #[builder(into)]
    pub nature: NatureName,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Hydrate, Reconcile)]
#[loro(root = "connections")]
pub struct Connections(HashMap<String, Connection>);

resource_id!(ConnectionId);
