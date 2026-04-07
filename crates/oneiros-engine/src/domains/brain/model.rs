use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Brain {
    #[builder(default = BrainId::new())]
    pub id: BrainId,
    #[builder(into)]
    pub name: BrainName,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "brains")]
pub struct Brains(HashMap<String, Brain>);

impl Brains {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, id: BrainId) -> Option<&Brain> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, brain: &Brain) -> Option<Brain> {
        self.0.insert(brain.id.to_string(), brain.clone())
    }

    pub fn remove(&mut self, brain_id: BrainId) -> Option<Brain> {
        self.0.remove(&brain_id.to_string())
    }
}

resource_name!(BrainName);
resource_id!(BrainId);
