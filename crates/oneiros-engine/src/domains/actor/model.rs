use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Actor {
    #[builder(default)]
    pub(crate) id: ActorId,
    pub(crate) tenant_id: TenantId,
    #[builder(into)]
    pub(crate) name: ActorName,
    #[builder(default = Timestamp::now(), into)]
    pub(crate) created_at: Timestamp,
}

impl Indexable<ActorId> for Actor {
    fn id(&self) -> ActorId {
        self.id
    }
}

pub(crate) type Actors = EntityIndex<ActorId, Actor>;

resource_id!(ActorId);
resource_name!(ActorName);
