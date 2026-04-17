use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Cognition {
    #[builder(default)]
    pub id: CognitionId,
    pub agent_id: AgentId,
    #[builder(into)]
    pub texture: TextureName,
    #[builder(into)]
    pub content: Content,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Clone, Default)]
pub struct Cognitions(HashMap<String, Cognition>);

impl Cognitions {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn values(&self) -> impl Iterator<Item = &Cognition> {
        self.0.values()
    }

    pub fn get(&self, id: CognitionId) -> Option<&Cognition> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, cognition: &Cognition) -> Option<Cognition> {
        self.0.insert(cognition.id.to_string(), cognition.clone())
    }

    pub fn remove(&mut self, cognition_id: CognitionId) -> Option<Cognition> {
        self.0.remove(&cognition_id.to_string())
    }
}

resource_id!(CognitionId);
