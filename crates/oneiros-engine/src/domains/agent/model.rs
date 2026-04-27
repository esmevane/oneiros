use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Agent {
    Current(AgentV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct AgentV1 {
    #[builder(default)]
    pub id: AgentId,
    #[builder(into)]
    pub name: AgentName,
    #[builder(into)]
    pub persona: PersonaName,
    #[builder(into)]
    pub description: Description,
    #[builder(into)]
    pub prompt: Prompt,
}

impl Agent {
    pub fn build_v1() -> AgentV1Builder {
        AgentV1::builder()
    }

    pub fn id(&self) -> AgentId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn name(&self) -> &AgentName {
        match self {
            Self::Current(v) => &v.name,
        }
    }

    pub fn persona(&self) -> &PersonaName {
        match self {
            Self::Current(v) => &v.persona,
        }
    }

    pub fn description(&self) -> &Description {
        match self {
            Self::Current(v) => &v.description,
        }
    }

    pub fn prompt(&self) -> &Prompt {
        match self {
            Self::Current(v) => &v.prompt,
        }
    }
}

#[derive(Clone, Default)]
pub struct Agents(HashMap<String, Agent>);

impl Agents {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, id: AgentId) -> Option<&Agent> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, agent: &Agent) -> Option<Agent> {
        self.0.insert(agent.id().to_string(), agent.clone())
    }

    pub fn remove(&mut self, agent_id: AgentId) -> Option<Agent> {
        self.0.remove(&agent_id.to_string())
    }

    pub fn find_by_name(&self, name: &AgentName) -> Option<&Agent> {
        self.0.values().find(|a| a.name() == name)
    }

    pub fn values(&self) -> impl Iterator<Item = &Agent> {
        self.0.values()
    }

    pub fn remove_by_name(&mut self, name: &AgentName) {
        self.0.retain(|_, a| a.name() != name);
    }
}

resource_id!(AgentId);
resource_name!(AgentName);

impl AgentName {
    pub fn normalize_with(&self, persona_name: &PersonaName) -> Self {
        let suffix = format!(".{persona_name}");
        if self.to_string().ends_with(&suffix) {
            self.clone()
        } else {
            Self::new(format!("{self}.{persona_name}"))
        }
    }
}
