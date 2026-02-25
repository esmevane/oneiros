use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnownEvent {
    pub id: EventId,
    pub timestamp: Timestamp,
    pub data: Events,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnknownEvent {
    pub id: EventId,
    pub timestamp: Timestamp,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    Known(KnownEvent),
    Unknown(UnknownEvent),
}

impl core::fmt::Display for Event {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Event::Known(KnownEvent {
                id,
                timestamp,
                data,
            }) => {
                write!(
                    f,
                    "{{ id: {}, timestamp: {}, data: {} }}",
                    id,
                    timestamp,
                    serde_json::to_string(&data).unwrap_or("Malformed event body".to_string())
                )
            }
            Event::Unknown(UnknownEvent {
                id,
                timestamp,
                data,
            }) => {
                write!(
                    f,
                    "{{ id: {}, timestamp: {}, data: {} }}",
                    id,
                    timestamp,
                    serde_json::to_string(&data).unwrap_or("Malformed event body".to_string())
                )
            }
        }
    }
}

domain_id!(EventId);
