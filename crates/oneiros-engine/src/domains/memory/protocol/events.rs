use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = MemoryEventsType, display = "kebab-case")]
pub enum MemoryEvents {
    MemoryAdded(MemoryAdded),
}

versioned! {
    pub enum MemoryAdded {
        V1 => {
            #[serde(flatten)] pub memory: Memory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_memory() -> Memory {
        Memory::builder()
            .agent_id(AgentId::new())
            .level("project")
            .content("A consolidated insight")
            .build()
    }

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&MemoryEventsType::MemoryAdded.to_string(), "memory-added");
    }

    #[test]
    fn memory_added_wire_format_is_flat() {
        let event = MemoryEvents::MemoryAdded(MemoryAdded::V1(MemoryAddedV1 {
            memory: sample_memory(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "memory-added");
        assert!(
            json["data"].get("memory").is_none(),
            "flatten must elide the memory envelope on the wire"
        );
        assert_eq!(json["data"]["level"], "project");
        assert_eq!(json["data"]["content"], "A consolidated insight");
        assert!(json["data"].get("id").is_some());
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }
}
