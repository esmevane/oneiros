use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// One row in the trail projection — the bridge between an event and an
/// entity it touched. The shape is a set-many join on both sides: an event
/// can emit many entities, an entity can be touched by many events. Initial
/// derivation rules keep each side small (typically 1:1 at creation time).
#[derive(Debug, Clone, PartialEq, Eq, Builder, Serialize, Deserialize, JsonSchema)]
pub(crate) struct TrailEntry {
    pub(crate) event_id: EventId,
    pub(crate) event_type: String,
    #[serde(rename = "ref")]
    pub(crate) entity_ref: RefToken,
    pub(crate) created_at: Timestamp,
}
