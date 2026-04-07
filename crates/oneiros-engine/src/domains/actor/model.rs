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

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "actors")]
pub struct Actors(HashMap<String, Actor>);

impl Actors {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, id: ActorId) -> Option<&Actor> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, actor: &Actor) -> Option<Actor> {
        self.0.insert(actor.id.to_string(), actor.clone())
    }

    pub fn remove(&mut self, actor_id: ActorId) -> Option<Actor> {
        self.0.remove(&actor_id.to_string())
    }
}

resource_id!(ActorId);
resource_name!(ActorName);
