use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = CognitionEventsType, display = "kebab-case")]
pub enum CognitionEvents {
    CognitionAdded(CognitionAdded),
}

versioned! {
    pub enum CognitionAdded {
        V1 => {
            #[serde(flatten)] pub cognition: Cognition,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_cognition() -> Cognition {
        Cognition::builder()
            .agent_id(AgentId::new())
            .texture("observation")
            .content("Something noticed")
            .build()
    }

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(
            &CognitionEventsType::CognitionAdded.to_string(),
            "cognition-added"
        );
    }

    #[test]
    fn cognition_added_wire_format_is_flat() {
        // V1 embeds `Cognition` with `#[serde(flatten)]`, so the wire shape stays
        // at the model fields and never gains a `cognition` envelope.
        let event = CognitionEvents::CognitionAdded(CognitionAdded::V1(CognitionAddedV1 {
            cognition: sample_cognition(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "cognition-added");
        assert!(
            json["data"].get("cognition").is_none(),
            "flatten must elide the cognition envelope on the wire"
        );
        assert_eq!(json["data"]["texture"], "observation");
        assert_eq!(json["data"]["content"], "Something noticed");
        assert!(json["data"].get("id").is_some());
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }
}
