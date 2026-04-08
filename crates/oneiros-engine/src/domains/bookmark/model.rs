use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A named timeline within a brain's canon.
///
/// Each bookmark points to a forked LoroDoc — an independent
/// line of development. The default bookmark is "main".
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Bookmark {
    #[builder(default)]
    pub id: BookmarkId,
    pub brain: BrainName,
    #[builder(into)]
    pub name: BookmarkName,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

resource_id!(BookmarkId);
resource_name!(BookmarkName);

impl BookmarkName {
    pub fn main() -> Self {
        Self::new("main")
    }
}
