use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Cognition {
    Current(CognitionV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CognitionV1 {
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

impl Cognition {
    pub fn build_v1() -> CognitionV1Builder {
        CognitionV1::builder()
    }

    pub fn id(&self) -> CognitionId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn agent_id(&self) -> AgentId {
        match self {
            Self::Current(v) => v.agent_id,
        }
    }

    pub fn texture(&self) -> &TextureName {
        match self {
            Self::Current(v) => &v.texture,
        }
    }

    pub fn content(&self) -> &Content {
        match self {
            Self::Current(v) => &v.content,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }
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
        self.0.insert(cognition.id().to_string(), cognition.clone())
    }

    pub fn remove(&mut self, cognition_id: CognitionId) -> Option<Cognition> {
        self.0.remove(&cognition_id.to_string())
    }
}

resource_id!(CognitionId);
