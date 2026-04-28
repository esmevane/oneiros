use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = BrainEventsType, display = "kebab-case")]
pub enum BrainEvents {
    BrainCreated(BrainCreated),
}

impl BrainEvents {
    pub fn maybe_brain(&self) -> Option<Brain> {
        match self {
            BrainEvents::BrainCreated(event) => event.clone().current().ok().map(|v| v.brain),
        }
    }
}

versioned! {
    pub enum BrainCreated {
        V1 => {
            #[serde(flatten)] pub brain: Brain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&BrainEventsType::BrainCreated.to_string(), "brain-created");
    }

    #[test]
    fn brain_created_wire_format_is_flat() {
        let brain = Brain::builder().name(BrainName::new("test-brain")).build();

        let event = BrainEvents::BrainCreated(BrainCreated::V1(BrainCreatedV1 {
            brain: brain.clone(),
        }));
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "brain-created");
        assert!(
            json["data"].get("brain").is_none(),
            "flatten must elide the brain envelope on the wire"
        );
        assert_eq!(json["data"]["id"], brain.id.to_string());
        assert_eq!(json["data"]["name"], "test-brain");
        assert!(json["data"].get("created_at").is_some());
    }
}
