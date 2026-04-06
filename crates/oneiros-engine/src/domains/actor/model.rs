use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Actor {
    #[builder(default)]
    pub id: ActorId,
    pub tenant_id: TenantId,
    #[builder(into)]
    pub name: ActorName,
    #[builder(default = Timestamp::now(), into)]
    pub created_at: Timestamp,
}

#[derive(Hydrate, Reconcile)]
#[loro(root = "actors")]
pub struct Actors(HashMap<String, Actor>);

resource_id!(ActorId);
resource_name!(ActorName);
