use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Experience {
    #[builder(default)]
    pub id: ExperienceId,
    pub agent_id: AgentId,
    #[builder(into)]
    pub sensation: SensationName,
    #[builder(into)]
    pub description: Description,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

impl Experience {
    pub fn ref_token(&self) -> RefToken {
        RefToken::from(Ref::experience(self.id))
    }
}

#[derive(Clone, Default)]
pub struct Experiences(HashMap<String, Experience>);

impl Experiences {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, id: ExperienceId) -> Option<&Experience> {
        self.0.get(&id.to_string())
    }

    pub fn values(&self) -> impl Iterator<Item = &Experience> {
        self.0.values()
    }

    pub fn get_mut(&mut self, id: ExperienceId) -> Option<&mut Experience> {
        self.0.get_mut(&id.to_string())
    }

    pub fn set(&mut self, experience: &Experience) -> Option<Experience> {
        self.0.insert(experience.id.to_string(), experience.clone())
    }

    pub fn remove(&mut self, experience_id: ExperienceId) -> Option<Experience> {
        self.0.remove(&experience_id.to_string())
    }
}

resource_id!(ExperienceId);
