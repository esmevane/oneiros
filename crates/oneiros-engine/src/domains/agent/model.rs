use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
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

impl Indexable<AgentId> for Agent {
    fn id(&self) -> AgentId {
        self.id
    }
}

pub(crate) type Agents = EntityIndex<AgentId, Agent>;

impl Agents {
    pub(crate) fn find_by_name(&self, name: &AgentName) -> Option<&Agent> {
        self.values().find(|a| a.name == *name)
    }

    pub(crate) fn remove_by_name(&mut self, name: &AgentName) {
        if let Some(agent) = self.find_by_name(name) {
            let id = agent.id;
            self.remove(&id);
        }
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
