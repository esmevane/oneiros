use bon::Builder;

use crate::*;

/// Apply a stored event to a bookmark's brain DB projections.
#[derive(Builder, Clone)]
pub struct ApplyBookmarkProjection {
    pub scope: Scope<AtBookmark>,
    #[builder(into)]
    pub stored: Box<StoredEvent>,
}

/// Clear and replay a bookmark's brain DB projections.
#[derive(Builder, Clone)]
pub struct ResetBookmarkProjection {
    pub scope: Scope<AtBookmark>,
}

/// Record a stored event into a bookmark's chronicle HAMT.
#[derive(Builder, Clone)]
pub struct RecordBookmarkChronicle {
    pub scope: Scope<AtBookmark>,
    #[builder(into)]
    pub stored: Box<StoredEvent>,
}

/// Clear and replay a bookmark's chronicle.
#[derive(Builder, Clone)]
pub struct ResetBookmarkChronicle {
    pub scope: Scope<AtBookmark>,
}

/// All bookmark-tier messages, flat. Routed per (brain, bookmark) by
/// the router; actors handle their own variants and no-op the rest.
#[derive(Clone)]
pub enum BookmarkMessage {
    ProjectionApply(ApplyBookmarkProjection),
    ProjectionReset(ResetBookmarkProjection),
    ChronicleRecord(RecordBookmarkChronicle),
    ChronicleReset(ResetBookmarkChronicle),
}

impl BookmarkMessage {
    pub fn scope(&self) -> &Scope<AtBookmark> {
        match self {
            Self::ProjectionApply(message) => &message.scope,
            Self::ProjectionReset(message) => &message.scope,
            Self::ChronicleRecord(message) => &message.scope,
            Self::ChronicleReset(message) => &message.scope,
        }
    }
}

collects_enum!(
    BookmarkMessage::ProjectionApply => ApplyBookmarkProjection,
    BookmarkMessage::ProjectionReset => ResetBookmarkProjection,
    BookmarkMessage::ChronicleRecord => RecordBookmarkChronicle,
    BookmarkMessage::ChronicleReset => ResetBookmarkChronicle,
);
