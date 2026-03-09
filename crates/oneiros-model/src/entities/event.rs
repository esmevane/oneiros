use serde::{Deserialize, Serialize};

use crate::*;

/// A pre-persistence event — has identity and payload, but no sequence number.
/// Created via [`NewEvent::new`] and promoted to [`KnownEvent`] by the store
/// after it assigns a sequence position.
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct NewEvent {
    pub id: EventId,
    pub timestamp: Timestamp,
    pub source: Source,
    pub data: Events,
}

impl NewEvent {
    pub fn new(data: Events, source: Source) -> Self {
        Self {
            id: EventId::new(),
            timestamp: Timestamp::now(),
            source,
            data,
        }
    }
}

/// A post-persistence event — carries a `sequence` number assigned by the
/// store, making its position in the log explicit.
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct KnownEvent {
    pub id: EventId,
    pub sequence: u64,
    pub timestamp: Timestamp,
    pub source: Source,
    pub data: Events,
}

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UnknownEvent {
    pub id: EventId,
    pub sequence: u64,
    pub timestamp: Timestamp,
    pub source: Source,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Event {
    Known(KnownEvent),
    Unknown(UnknownEvent),
    New(NewEvent),
}

impl core::fmt::Display for Event {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Event::Known(KnownEvent {
                id,
                sequence,
                timestamp,
                source,
                data,
            }) => {
                write!(
                    f,
                    "{{ id: {}, sequence: {}, timestamp: {}, source: {:?}, data: {} }}",
                    id,
                    sequence,
                    timestamp,
                    source,
                    serde_json::to_string(&data)
                        .unwrap_or_else(|_| "Malformed event body".to_string())
                )
            }
            Event::Unknown(UnknownEvent {
                id,
                sequence,
                timestamp,
                source,
                data,
            }) => {
                write!(
                    f,
                    "{{ id: {}, sequence: {}, timestamp: {}, source: {:?}, data: {} }}",
                    id,
                    sequence,
                    timestamp,
                    source,
                    serde_json::to_string(&data)
                        .unwrap_or_else(|_| "Malformed event body".to_string())
                )
            }
            Event::New(NewEvent {
                id,
                timestamp,
                source,
                data,
            }) => {
                write!(
                    f,
                    "{{ id: {}, timestamp: {}, source: {:?}, data: {} }}",
                    id,
                    timestamp,
                    source,
                    serde_json::to_string(&data)
                        .unwrap_or_else(|_| "Malformed event body".to_string())
                )
            }
        }
    }
}

domain_id!(EventId);

#[cfg(test)]
mod tests {
    use super::*;

    fn test_source() -> Source {
        Source::default()
    }

    fn test_events() -> Events {
        Events::Lifecycle(LifecycleEvents::Woke(SelectAgentByName {
            name: AgentName::new("test-agent"),
        }))
    }

    #[test]
    fn new_event_has_no_sequence() {
        let event = NewEvent::new(test_events(), test_source());

        // NewEvent has id, timestamp, source, and data — but no sequence.
        // The type itself enforces this: there is no sequence field to access.
        // Verify by serializing: the JSON must contain "id" but not "sequence".
        let serialized = serde_json::to_string(&event).expect("serializes");
        assert!(serialized.contains("\"id\""));
        assert!(!serialized.contains("\"sequence\""));
    }

    #[test]
    fn known_event_has_sequence() {
        let event = KnownEvent {
            id: EventId::new(),
            sequence: 42,
            timestamp: Timestamp::now(),
            source: test_source(),
            data: test_events(),
        };

        assert_eq!(event.sequence, 42);
    }

    #[test]
    fn known_event_serializes_with_sequence() {
        let event = KnownEvent {
            id: EventId::new(),
            sequence: 7,
            timestamp: Timestamp::now(),
            source: test_source(),
            data: test_events(),
        };

        let json = serde_json::to_string(&event).expect("serializes");
        assert!(json.contains("\"sequence\":7"));
    }

    #[test]
    fn new_event_deserializes_as_event_new_variant() {
        let new = NewEvent::new(test_events(), test_source());
        let json = serde_json::to_string(&new).expect("serializes");
        let event: Event = serde_json::from_str(&json).expect("deserializes");

        match event {
            Event::New(deserialized) => {
                assert_eq!(deserialized.id, new.id);
            }
            _ => panic!("expected Event::New variant, got: {event}"),
        }
    }

    #[test]
    fn known_event_deserializes_with_sequence() {
        let original = KnownEvent {
            id: EventId::new(),
            sequence: 99,
            timestamp: Timestamp::now(),
            source: test_source(),
            data: test_events(),
        };

        let json = serde_json::to_string(&original).expect("serializes");
        let roundtripped: KnownEvent = serde_json::from_str(&json).expect("deserializes");

        assert_eq!(original.id, roundtripped.id);
        assert_eq!(roundtripped.sequence, 99);
    }
}
