use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub(crate) struct Agent {
    #[builder(default)]
    pub(crate) id: AgentId,
    #[builder(into)]
    pub(crate) name: AgentName,
    #[builder(into)]
    pub(crate) persona: PersonaName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "agents")]
pub(crate) struct Agents(HashMap<String, Agent>);

impl Agents {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, id: AgentId) -> Option<&Agent> {
        self.0.get(&id.to_string())
    }

    pub(crate) fn set(&mut self, agent: &Agent) -> Option<Agent> {
        self.0.insert(agent.id.to_string(), agent.clone())
    }

    pub(crate) fn remove(&mut self, agent_id: AgentId) -> Option<Agent> {
        self.0.remove(&agent_id.to_string())
    }

    pub(crate) fn find_by_name(&self, name: &AgentName) -> Option<&Agent> {
        self.0.values().find(|a| a.name == *name)
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &Agent> {
        self.0.values()
    }

    pub(crate) fn remove_by_name(&mut self, name: &AgentName) {
        self.0.retain(|_, a| a.name != *name);
    }
}

resource_id!(AgentId);
resource_name!(AgentName);

impl AgentName {
    pub(crate) fn normalize_with(&self, persona_name: &PersonaName) -> Self {
        let suffix = format!(".{persona_name}");
        if self.to_string().ends_with(&suffix) {
            self.clone()
        } else {
            Self::new(format!("{self}.{persona_name}"))
        }
    }
}
