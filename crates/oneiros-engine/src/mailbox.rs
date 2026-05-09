//! Global mailbox.
//!
//! `Mailbox` is the cloneable handle held by services and actors. Anyone
//! can `tell` any [`Message`]; the mailbox dispatches by matching the
//! variant chain to the right per-actor inbox.
//!
//! There is no separate router task. The mailbox holds a clone of every
//! per-actor sender and routes synchronously in `tell`. Each per-actor
//! inbox is consumed by a singleton actor task spawned during
//! [`Mailbox::spawn`].

use crate::*;

/// Cloneable, fire-and-forget handle to the global bus.
///
/// Holding a `Mailbox` is enough to emit any [`Message`] into the
/// system. Services hold one (typically through `ServerState`); actors
/// receive one at spawn time so they can address other actors without
/// holding sibling handles.
#[derive(Clone)]
pub(crate) struct Mailbox {
    system_log: SystemLogMailbox,
    system_projection: SystemProjectionMailbox,
    project_log: ProjectLogMailbox,
    project_import: ProjectImportMailbox,
    bookmark_projections: BookmarkProjectionsMailbox,
    bookmark_chronicle: BookmarkChronicleMailbox,
}

impl Mailbox {
    /// Spawn the topology and return the cloneable mailbox.
    ///
    /// Each per-actor inbox is consumed by a singleton task; each actor
    /// receives a clone of the mailbox so it can emit follow-ups without
    /// holding handles to siblings.
    pub(crate) fn spawn(canons: CanonIndex) -> Self {
        let (system_log, system_log_inbox) = SystemLogMailbox::open();
        let (system_projection, system_projection_inbox) = SystemProjectionMailbox::open();
        let (project_log, project_log_inbox) = ProjectLogMailbox::open();
        let (project_import, project_import_inbox) = ProjectImportMailbox::open();
        let (bookmark_projections, bookmark_projections_inbox) = BookmarkProjectionsMailbox::open();
        let (bookmark_chronicle, bookmark_chronicle_inbox) = BookmarkChronicleMailbox::open();

        let mailbox = Self {
            system_log,
            system_projection,
            project_log,
            project_import,
            bookmark_projections,
            bookmark_chronicle,
        };

        SystemLogActor::spawn(system_log_inbox, mailbox.clone());
        SystemProjectionActor::spawn(system_projection_inbox);
        ProjectLogActor::spawn(project_log_inbox, mailbox.clone());
        ProjectImportActor::spawn(project_import_inbox, mailbox.clone());
        BookmarkProjectionsActor::spawn(bookmark_projections_inbox, canons.clone());
        BookmarkChronicleActor::spawn(bookmark_chronicle_inbox, canons);

        mailbox
    }

    /// Send a message into the bus. `Into<Message>` lets callers pass
    /// per-tier messages or action structs directly without wrapping
    /// at the call site.
    pub(crate) fn tell(&self, message: impl Into<Message>) {
        match message.into() {
            Message::System(message) => match message {
                SystemMessage::LogAppend(_) => {
                    self.system_log.tell(message);
                }
                SystemMessage::ProjectionApply(_)
                | SystemMessage::ProjectionMigrate(_)
                | SystemMessage::ProjectionReset(_) => self.system_projection.tell(message),
            },
            Message::Project(message) => match message {
                ProjectMessage::LogAppend(_) => {
                    self.project_log.tell(message);
                }
                ProjectMessage::ImportEvent(_) => {
                    self.project_import.tell(message);
                }
            },
            Message::Bookmark(message) => match message {
                BookmarkMessage::ProjectionApply(_) | BookmarkMessage::ProjectionReset(_) => {
                    self.bookmark_projections.tell(message);
                }
                BookmarkMessage::ChronicleRecord(_) | BookmarkMessage::ChronicleReset(_) => {
                    self.bookmark_chronicle.tell(message);
                }
            },
        }
    }
}
