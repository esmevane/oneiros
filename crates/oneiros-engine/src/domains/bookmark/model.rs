use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A named timeline within a brain's canon.
///
/// Each bookmark points to a forked LoroDoc — an independent
/// line of development. The default bookmark is "main".
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Bookmark {
    #[builder(default)]
    pub(crate) id: BookmarkId,
    pub(crate) brain: BrainName,
    #[builder(into)]
    pub(crate) name: BookmarkName,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

resource_id!(BookmarkId);
resource_name!(BookmarkName);

impl BookmarkName {
    pub(crate) fn main() -> Self {
        Self::new("main")
    }
}
