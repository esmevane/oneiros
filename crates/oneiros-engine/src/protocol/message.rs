use crate::*;

/// Top-level super-enum addressing every actor in the system.
///
/// Composed hierarchically: top → tier → per-actor protocol. The router
/// dispatches by matching the variant chain.
#[derive(Clone)]
pub(crate) enum Message {
    System(SystemMessage),
    Project(ProjectMessage),
    Bookmark(BookmarkMessage),
}

collects_enum!(
    Message::System => SystemMessage,
    Message::Project => ProjectMessage,
    Message::Bookmark => BookmarkMessage,
);
