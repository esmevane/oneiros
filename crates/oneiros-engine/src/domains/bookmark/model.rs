use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A named timeline within a brain's canon.
///
/// Each bookmark points to a forked LoroDoc — an independent
/// line of development. The default bookmark is "main".
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Bookmark {
    Current(BookmarkV1),
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct BookmarkV1 {
    #[builder(default)]
    pub id: BookmarkId,
    pub brain: BrainName,
    #[builder(into)]
    pub name: BookmarkName,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

impl Bookmark {
    pub fn build_v1() -> BookmarkV1Builder {
        BookmarkV1::builder()
    }

    pub fn id(&self) -> BookmarkId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn brain(&self) -> &BrainName {
        match self {
            Self::Current(v) => &v.brain,
        }
    }

    pub fn name(&self) -> &BookmarkName {
        match self {
            Self::Current(v) => &v.name,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }
}

resource_id!(BookmarkId);
resource_name!(BookmarkName);

impl BookmarkName {
    pub fn main() -> Self {
        Self::new("main")
    }
}
