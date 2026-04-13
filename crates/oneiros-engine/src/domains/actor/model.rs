use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub(crate) struct Actor {
    #[builder(default)]
    pub(crate) id: ActorId,
    pub(crate) tenant_id: TenantId,
    #[builder(into)]
    pub(crate) name: ActorName,
    #[builder(default = Timestamp::now(), into)]
    pub(crate) created_at: Timestamp,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "actors")]
pub(crate) struct Actors(HashMap<String, Actor>);

impl Actors {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, id: ActorId) -> Option<&Actor> {
        self.0.get(&id.to_string())
    }

    pub(crate) fn set(&mut self, actor: &Actor) -> Option<Actor> {
        self.0.insert(actor.id.to_string(), actor.clone())
    }

    pub(crate) fn remove(&mut self, actor_id: ActorId) -> Option<Actor> {
        self.0.remove(&actor_id.to_string())
    }
}

resource_id!(ActorId);
resource_name!(ActorName);
