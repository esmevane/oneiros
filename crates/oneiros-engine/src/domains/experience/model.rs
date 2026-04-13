use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub(crate) struct Experience {
    #[builder(default)]
    pub(crate) id: ExperienceId,
    pub(crate) agent_id: AgentId,
    #[builder(into)]
    pub(crate) sensation: SensationName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "experiences")]
pub(crate) struct Experiences(HashMap<String, Experience>);

impl Experiences {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, id: ExperienceId) -> Option<&Experience> {
        self.0.get(&id.to_string())
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &Experience> {
        self.0.values()
    }

    pub(crate) fn get_mut(&mut self, id: ExperienceId) -> Option<&mut Experience> {
        self.0.get_mut(&id.to_string())
    }

    pub(crate) fn set(&mut self, experience: &Experience) -> Option<Experience> {
        self.0.insert(experience.id.to_string(), experience.clone())
    }

    pub(crate) fn remove(&mut self, experience_id: ExperienceId) -> Option<Experience> {
        self.0.remove(&experience_id.to_string())
    }
}

resource_id!(ExperienceId);
