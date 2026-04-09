use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = BookmarkResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BookmarkResponse {
    Created(BookmarkCreated),
    Forked(BookmarkForked),
    Switched(BookmarkSwitched),
    Merged(BookmarkMerged),
    Bookmarks(Listed<Bookmark>),
}
