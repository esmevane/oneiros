use bon::Builder;

use crate::*;

/// Append a fresh event to a project's events.db. The bookmark scope is
/// preserved so post-storage downstream addresses the right bookmark.
#[derive(Builder, Clone)]
pub(crate) struct AppendProjectLog {
    pub(crate) scope: Scope<AtBookmark>,
    #[builder(into)]
    pub(crate) event: Box<NewEvent>,
}

/// Insert-or-ignore a foreign stored event into the project's events.db.
/// Bookmark scope is preserved so downstream addresses the right bookmark.
#[derive(Builder, Clone)]
pub(crate) struct ImportProjectEvent {
    pub(crate) scope: Scope<AtBookmark>,
    #[builder(into)]
    pub(crate) stored: Box<StoredEvent>,
}

/// All project-tier messages, flat. Routed per-project by the router;
/// actors handle their own variants and no-op the rest.
#[derive(Clone)]
pub(crate) enum ProjectMessage {
    LogAppend(AppendProjectLog),
    ImportEvent(ImportProjectEvent),
}

collects_enum!(
    ProjectMessage::LogAppend => AppendProjectLog,
    ProjectMessage::ImportEvent => ImportProjectEvent,
);
