use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Experience {
    Current(ExperienceV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ExperienceV1 {
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
    pub fn build_v1() -> ExperienceV1Builder {
        ExperienceV1::builder()
    }

    pub fn id(&self) -> ExperienceId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn agent_id(&self) -> AgentId {
        match self {
            Self::Current(v) => v.agent_id,
        }
    }

    pub fn sensation(&self) -> &SensationName {
        match self {
            Self::Current(v) => &v.sensation,
        }
    }

    pub fn description(&self) -> &Description {
        match self {
            Self::Current(v) => &v.description,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }

    pub fn set_description(&mut self, description: Description) {
        match self {
            Self::Current(v) => v.description = description,
        }
    }

    pub fn set_sensation(&mut self, sensation: SensationName) {
        match self {
            Self::Current(v) => v.sensation = sensation,
        }
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
        self.0
            .insert(experience.id().to_string(), experience.clone())
    }

    pub fn remove(&mut self, experience_id: ExperienceId) -> Option<Experience> {
        self.0.remove(&experience_id.to_string())
    }
}

resource_id!(ExperienceId);
