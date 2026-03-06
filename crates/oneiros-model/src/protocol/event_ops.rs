use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportEvent {
    Valid {
        id: EventId,
        source: Source,
        timestamp: String,
        data: serde_json::Value,
    },
    Unsourced {
        id: EventId,
        timestamp: String,
        data: serde_json::Value,
    },
}

impl ImportEvent {
    pub fn with_source(self, source: Source) -> Self {
        match self {
            ImportEvent::Unsourced {
                id,
                timestamp,
                data,
            } => ImportEvent::Valid {
                id,
                source,
                timestamp,
                data,
            },
            valid @ ImportEvent::Valid { .. } => valid,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResponse {
    pub imported: usize,
    pub replayed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResponse {
    pub replayed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectEventById {
    pub id: EventId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum EventRequests {
    ImportEvents(Vec<ImportEvent>),
    ReplayEvents,
    ListEvents,
    GetEvent(SelectEventById),
    ExportEvents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum EventResponses {
    Imported(ImportResponse),
    Replayed(ReplayResponse),
    Listed(Vec<Event>),
    Found(Event),
    Exported(Vec<Event>),
}
