use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A standing lens-filtered view over a project's event log.
///
/// A slice is not bound to a particular bookmark — it materializes
/// its own chronicle and projection state independently. Bookmarking
/// a slice creates a transportable snapshot for sharing.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Slice {
    #[builder(default)]
    pub(crate) id: SliceId,
    #[builder(into)]
    pub(crate) name: SliceName,
    #[builder(into)]
    pub(crate) lens_expr: String,
    #[builder(default = 0)]
    pub(crate) event_count: u64,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

resource_id!(SliceId);
resource_name!(SliceName);
