use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacyEvent {
    pub id: EventId,
    pub timestamp: Timestamp,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    Legacy(LegacyEvent),
}

impl core::fmt::Display for Event {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Event::Legacy(LegacyEvent {
                id,
                timestamp,
                data,
            }) => {
                write!(
                    f,
                    "Event {{ id: {}, timestamp: {}, data: {} }}",
                    id,
                    timestamp,
                    serde_json::to_string_pretty(&data)
                        .unwrap_or("Malformed event body".to_string())
                )
            }
        }
    }
}

domain_id!(EventId);
