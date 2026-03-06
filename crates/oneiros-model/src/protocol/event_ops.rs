use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportEvent {
    pub id: EventId,
    pub source: Source,
    pub timestamp: String,
    pub data: serde_json::Value,
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
