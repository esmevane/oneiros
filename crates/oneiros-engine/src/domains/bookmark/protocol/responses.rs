use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BookmarkResponse {
    Created(BookmarkCreated),
    Forked(BookmarkForked),
    Switched(BookmarkSwitched),
    Merged(BookmarkMerged),
    Bookmarks(Listed<Bookmark>),
}
