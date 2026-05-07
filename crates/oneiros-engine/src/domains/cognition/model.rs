use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Cognition {
    #[builder(default)]
    pub(crate) id: CognitionId,
    pub(crate) agent_id: AgentId,
    #[builder(into)]
    pub(crate) texture: TextureName,
    #[builder(into)]
    pub(crate) content: Content,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Cognition {
    pub(crate) fn ref_token(&self) -> RefToken {
        RefToken::from(Ref::cognition(self.id))
    }
}

#[derive(Clone, Default)]
pub(crate) struct Cognitions(HashMap<String, Cognition>);

impl Cognitions {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &Cognition> {
        self.0.values()
    }

    pub(crate) fn get(&self, id: CognitionId) -> Option<&Cognition> {
        self.0.get(&id.to_string())
    }

    pub(crate) fn set(&mut self, cognition: &Cognition) -> Option<Cognition> {
        self.0.insert(cognition.id.to_string(), cognition.clone())
    }

    pub(crate) fn remove(&mut self, cognition_id: CognitionId) -> Option<Cognition> {
        self.0.remove(&cognition_id.to_string())
    }
}

resource_id!(CognitionId);
