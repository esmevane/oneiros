use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    Legacy {
        id: EventId,
        timestamp: Timestamp,
        data: Resource,
    },
}

domain_id!(EventId);
