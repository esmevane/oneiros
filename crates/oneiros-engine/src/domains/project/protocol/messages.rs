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

/// Reset the project's events.db. (No-op stub for now — durable record.)
#[derive(Builder, Clone)]
pub(crate) struct ResetProjectLog {
    pub(crate) scope: Scope<AtProject>,
}

/// Insert-or-ignore a foreign stored event into the project's events.db.
/// Bookmark scope is preserved so downstream addresses the right bookmark.
#[derive(Builder, Clone)]
pub(crate) struct ImportProjectEvent {
    pub(crate) scope: Scope<AtBookmark>,
    #[builder(into)]
    pub(crate) stored: Box<StoredEvent>,
}

/// Reset the project's events.db ingest state. (No-op for now.)
#[derive(Builder, Clone)]
pub(crate) struct ResetProjectImport {
    pub(crate) scope: Scope<AtProject>,
}

/// All project-tier messages, flat. Routed per-brain by the router;
/// actors handle their own variants and no-op the rest.
#[derive(Clone)]
pub(crate) enum ProjectMessage {
    LogAppend(AppendProjectLog),
    LogReset(ResetProjectLog),
    ImportEvent(ImportProjectEvent),
    ImportReset(ResetProjectImport),
}

impl ProjectMessage {
    pub(crate) fn brain(&self) -> &BrainName {
        match self {
            Self::LogAppend(message) => &message.scope.project().name,
            Self::LogReset(message) => &message.scope.project().name,
            Self::ImportEvent(message) => &message.scope.project().name,
            Self::ImportReset(message) => &message.scope.project().name,
        }
    }
}

collects_enum!(
    ProjectMessage::LogAppend => AppendProjectLog,
    ProjectMessage::LogReset => ResetProjectLog,
    ProjectMessage::ImportEvent => ImportProjectEvent,
    ProjectMessage::ImportReset => ResetProjectImport,
);
