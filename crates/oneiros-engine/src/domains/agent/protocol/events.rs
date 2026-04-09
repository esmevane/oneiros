use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = AgentEventsType, display = "kebab-case")]
pub enum AgentEvents {
    AgentCreated(Agent),
    AgentUpdated(Agent),
    AgentRemoved(AgentRemoved),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (AgentEventsType::AgentCreated, "agent-created"),
            (AgentEventsType::AgentUpdated, "agent-updated"),
            (AgentEventsType::AgentRemoved, "agent-removed"),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRemoved {
    pub name: AgentName,
}
