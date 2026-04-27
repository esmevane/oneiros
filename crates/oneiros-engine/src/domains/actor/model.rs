use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Actor {
    Current(ActorV1),
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ActorV1 {
    #[builder(default)]
    pub id: ActorId,
    pub tenant_id: TenantId,
    #[builder(into)]
    pub name: ActorName,
    #[builder(default = Timestamp::now(), into)]
    pub created_at: Timestamp,
}

impl Actor {
    pub fn build_v1() -> ActorV1Builder {
        ActorV1::builder()
    }

    pub fn id(&self) -> ActorId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn tenant_id(&self) -> TenantId {
        match self {
            Self::Current(v) => v.tenant_id,
        }
    }

    pub fn name(&self) -> &ActorName {
        match self {
            Self::Current(v) => &v.name,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }
}

#[derive(Clone, Default)]
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
        self.0.insert(actor.id().to_string(), actor.clone())
    }

    pub fn remove(&mut self, actor_id: ActorId) -> Option<Actor> {
        self.0.remove(&actor_id.to_string())
    }
}

resource_id!(ActorId);
resource_name!(ActorName);
