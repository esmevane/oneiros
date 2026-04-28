use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = AgentEventsType, display = "kebab-case")]
pub enum AgentEvents {
    AgentCreated(AgentCreated),
    AgentUpdated(AgentUpdated),
    AgentRemoved(AgentRemoved),
}

impl AgentEvents {
    pub fn maybe_agent(&self) -> Option<Agent> {
        match self {
            AgentEvents::AgentCreated(event) => event.clone().current().ok().map(|v| v.agent),
            AgentEvents::AgentUpdated(event) => event.clone().current().ok().map(|v| v.agent),
            AgentEvents::AgentRemoved(_) => None,
        }
    }
}

versioned! {
    pub enum AgentCreated {
        V1 => {
            #[serde(flatten)] pub agent: Agent,
        },
        V0 => {
            #[builder(into)] pub name: AgentName,
            #[builder(into)] pub persona: PersonaName,
            #[builder(into)] pub description: Description,
            #[builder(into)] pub prompt: Prompt,
        }
    }
}

upcast_versions! {
    AgentCreatedV0 { name, persona, description, prompt } => AgentCreatedV1 {
        agent: Agent::builder()
            .name(name)
            .persona(persona)
            .description(description)
            .prompt(prompt)
            .build()
    }
}

versioned! {
    pub enum AgentUpdated {
        V1 => { #[serde(flatten)] pub agent: Agent },
    }
}

versioned! {
    pub enum AgentRemoved {
        V1 => { #[builder(into)] pub name: AgentName },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_agent() -> Agent {
        Agent::builder()
            .name("test.process")
            .persona("process")
            .description("desc")
            .prompt("prompt")
            .build()
    }

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

    #[test]
    fn agent_created_wire_format_is_flat() {
        // V1 embeds `Agent` with `#[serde(flatten)]`, so the wire shape stays
        // at the model fields and never gains an `agent` envelope.
        let event = AgentEvents::AgentCreated(AgentCreated::V1(AgentCreatedV1 {
            agent: sample_agent(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "agent-created");
        assert!(
            json["data"].get("agent").is_none(),
            "flatten must elide the agent envelope on the wire"
        );
        assert_eq!(json["data"]["name"], "test.process");
        assert_eq!(json["data"]["persona"], "process");
        assert!(json["data"].get("id").is_some());
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }

    #[test]
    fn agent_removed_round_trips_through_v1_layer() {
        let original = AgentEvents::AgentRemoved(AgentRemoved::V1(AgentRemovedV1 {
            name: AgentName::new("test.process"),
        }));

        let json = serde_json::to_string(&original).unwrap();
        let decoded: AgentEvents = serde_json::from_str(&json).unwrap();

        match decoded {
            AgentEvents::AgentRemoved(removed) => {
                let v1 = removed.current().unwrap();
                assert_eq!(v1.name.to_string(), "test.process");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn deserialize_accepts_pre_version_wire_shape() {
        // Old (pre-versioning) wire shape was just the bare struct fields.
        // Untagged V1 must accept this byte-identical shape.
        let pre_version_json = serde_json::json!({
            "type": "agent-removed",
            "data": { "name": "test.process" }
        });

        let event: AgentEvents = serde_json::from_value(pre_version_json).unwrap();
        match event {
            AgentEvents::AgentRemoved(AgentRemoved::V1(v)) => {
                assert_eq!(v.name.to_string(), "test.process");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn agent_created_v0_upcasts_to_v1_via_agent() {
        // The V0 → V1 upcast constructs an `Agent` inline using the legacy
        // V0 fields and a fresh `id`.
        let v0 = AgentCreatedV0 {
            name: AgentName::new("legacy.process"),
            persona: PersonaName::new("process"),
            description: Description::new("legacy"),
            prompt: Prompt::new(""),
        };
        let v1: AgentCreatedV1 = v0.into();
        assert_eq!(v1.agent.name.to_string(), "legacy.process");
        assert_eq!(v1.agent.persona.to_string(), "process");
    }
}
