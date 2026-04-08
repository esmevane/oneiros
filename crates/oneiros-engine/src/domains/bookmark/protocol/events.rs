use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum BookmarkEvents {
    BookmarkCreated(BookmarkCreated),
    BookmarkForked(BookmarkForked),
    BookmarkSwitched(BookmarkSwitched),
    BookmarkMerged(BookmarkMerged),
}

/// Genesis — a bookmark comes into existence (e.g. "main" at brain init).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkCreated {
    pub brain: BrainName,
    pub name: BookmarkName,
}

/// Derivation — a new bookmark forked from an existing one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkForked {
    pub brain: BrainName,
    pub name: BookmarkName,
    pub from: BookmarkName,
}

/// Navigation — the active bookmark changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkSwitched {
    pub brain: BrainName,
    pub name: BookmarkName,
}

/// Convergence — one bookmark's changes merged into another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkMerged {
    pub brain: BrainName,
    pub source: BookmarkName,
    pub target: BookmarkName,
}
