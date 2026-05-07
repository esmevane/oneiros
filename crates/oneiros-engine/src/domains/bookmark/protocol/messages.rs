use bon::Builder;

use crate::*;

/// Apply a stored event to a bookmark's brain DB projections.
#[derive(Builder, Clone)]
pub(crate) struct ApplyBookmarkProjection {
    pub(crate) scope: Scope<AtBookmark>,
    #[builder(into)]
    pub(crate) stored: Box<StoredEvent>,
}

/// Clear and replay a bookmark's brain DB projections.
#[derive(Builder, Clone)]
pub(crate) struct ResetBookmarkProjection {
    pub(crate) scope: Scope<AtBookmark>,
}

/// Record a stored event into a bookmark's chronicle HAMT.
#[derive(Builder, Clone)]
pub(crate) struct RecordBookmarkChronicle {
    pub(crate) scope: Scope<AtBookmark>,
    #[builder(into)]
    pub(crate) stored: Box<StoredEvent>,
}

/// Clear and replay a bookmark's chronicle.
#[derive(Builder, Clone)]
pub(crate) struct ResetBookmarkChronicle {
    pub(crate) scope: Scope<AtBookmark>,
}

/// All bookmark-tier messages, flat. Routed per (brain, bookmark) by
/// the router; actors handle their own variants and no-op the rest.
#[derive(Clone)]
pub(crate) enum BookmarkMessage {
    ProjectionApply(ApplyBookmarkProjection),
    ProjectionReset(ResetBookmarkProjection),
    ChronicleRecord(RecordBookmarkChronicle),
    ChronicleReset(ResetBookmarkChronicle),
}

impl BookmarkMessage {
    pub(crate) fn scope(&self) -> &Scope<AtBookmark> {
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
