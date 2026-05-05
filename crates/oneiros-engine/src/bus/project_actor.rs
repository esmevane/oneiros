//! `ProjectActor` — owns one project's event log and the routing tree
//! beneath it.
//!
//! One actor per brain (project). Spawned lazily by the `HostActor` on
//! the first project- or bookmark-tier message for that brain. Holds
//! lazy registries of `BookmarkMailbox` and `ChronicleMailbox` keyed by
//! `BookmarkName`; spawns those leaves on first sighting.
//!
//! The actor's domain has two paths into the same downstream:
//!
//! - **Append** — a local `Bookmark` message with `Event::New(...)`.
//!   The actor appends to the project event log, builds a `Stored`,
//!   and forwards to the bookmark + chronicle children.
//! - **Forward** — a `Bookmark` message with `Event::Stored(...)`,
//!   typically delivered by the `InboundActor` after a foreign
//!   ingest. No append; just route the Stored to the children. This
//!   keeps "events.db ownership" + "child registry" in one actor
//!   without forking the input on event variant.
//!
//! Project-tier messages (events that apply to all bookmarks in a
//! project) are not yet wired — services today emit at the bookmark
//! tier. The variant is logged at debug and dropped until something
//! needs it.

use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::*;

/// Domain messages forwarded into a project actor by the host actor
/// (and, in the case of `Forward`, by the inbound actor).
#[derive(Clone)]
pub enum ProjectDomain {
    /// Project-tier message — currently log-and-drop.
    Project(Message<AtProject>),
    /// Bookmark-tier `New` event — actor appends to events.db, then
    /// forwards a `Stored` to the bookmark + chronicle children.
    Bookmark(Message<AtBookmark>),
    /// Bookmark-tier `Stored` event arriving post-ingest from the
    /// inbound actor — actor only forwards to children, no append.
    Forward(Message<AtBookmark>),
}

/// Send-side handle held by the `HostActor`. Cloneable.
#[derive(Clone)]
pub struct ProjectMailbox {
    tx: mpsc::UnboundedSender<LifecycleMessage<ProjectDomain>>,
}

impl ProjectMailbox {
    pub fn tell(&self, message: LifecycleMessage<ProjectDomain>) {
        if let Err(err) = self.tx.send(message) {
            tracing::warn!(error = %err, "project actor: receiver closed; message dropped");
        }
    }

    pub fn tell_domain(&self, message: ProjectDomain) {
        self.tell(LifecycleMessage::Domain(message));
    }
}

pub struct ProjectActor {
    brain: BrainName,
    /// Project-tier scope composed at spawn time. The actor owns this
    /// because the project event log is project-tier work — it
    /// shouldn't reach through a bookmark's per-request scope to do
    /// what's structurally its own job.
    scope: Scope<AtProject>,
    canons: CanonIndex,
    bookmarks: HashMap<BookmarkName, BookmarkMailbox>,
}

impl ProjectActor {
    /// Spawn a project actor for one brain. The host actor calls this
    /// the first time a project- or bookmark-scoped message arrives for
    /// the brain.
    pub fn spawn(brain: BrainName, scope: Scope<AtProject>, canons: CanonIndex) -> ProjectMailbox {
        let (tx, rx) = mpsc::unbounded_channel();
        let actor = Self {
            brain,
            scope,
            canons,
            bookmarks: HashMap::new(),
        };
        tokio::spawn(actor.run(rx));
        ProjectMailbox { tx }
    }

    async fn run(mut self, mut rx: mpsc::UnboundedReceiver<LifecycleMessage<ProjectDomain>>) {
        while let Some(message) = rx.recv().await {
            match message {
                LifecycleMessage::Domain(ProjectDomain::Bookmark(message)) => {
                    if let Err(err) = self.handle_append(message).await {
                        tracing::error!(
                            brain = %self.brain,
                            ?err,
                            "project actor: append failed",
                        );
                    }
                }
                LifecycleMessage::Domain(ProjectDomain::Forward(message)) => {
                    if let Err(err) = self.handle_forward(message) {
                        tracing::error!(
                            brain = %self.brain,
                            ?err,
                            "project actor: forward failed",
                        );
                    }
                }
                LifecycleMessage::Domain(ProjectDomain::Project(_)) => {
                    tracing::debug!(
                        brain = %self.brain,
                        "project actor: project-tier message dropped — no callers yet",
                    );
                }
                LifecycleMessage::Reset => {
                    // Reset cascades — every known bookmark child
                    // resets and rebuilds. The project actor itself
                    // has no state to wipe.
                    for mb in self.bookmarks.values() {
                        mb.reset();
                    }
                }
            }
        }
    }

    async fn handle_append(&mut self, message: Message<AtBookmark>) -> Result<(), EventError> {
        let Message {
            scope: req_scope,
            event,
        } = message;
        let new_event = match event {
            Event::New(boxed) => *boxed,
            // Stored notifications and parsing variants don't enter
            // the append path. Stored is handled via Forward; the
            // others are not bus traffic at all.
            _ => return Ok(()),
        };

        let bookmark = req_scope.bookmark().name.clone();

        // Project-tier append uses the actor's own project-tier scope.
        // The bookmark DB ATTACHes events read-only for query
        // convenience, but writes happen here, on the project tier.
        let events_db = EventsDb::open(&self.scope).await?;
        EventLog::new(&events_db).init()?;
        let stored = EventLog::new(&events_db).append(&new_event)?;

        let stored_message = Message {
            scope: req_scope,
            event: Event::Stored(Box::new(stored)),
        };

        self.fan_out(&bookmark, stored_message)
    }

    /// Forward a pre-stored event to bookmark + chronicle children
    /// without appending. Used for events that the inbound actor has
    /// already imported.
    fn handle_forward(&mut self, message: Message<AtBookmark>) -> Result<(), EventError> {
        // Sanity: only Stored makes sense here.
        if !matches!(message.event, Event::Stored(_)) {
            return Ok(());
        }
        let bookmark = message.scope.bookmark().name.clone();
        self.fan_out(&bookmark, message)
    }

    /// Spawn the bookmark actor for `bookmark` if needed and forward
    /// the stored notification to it. The bookmark actor handles both
    /// projection apply and chronicle record per-event in sequence.
    fn fan_out(
        &mut self,
        bookmark: &BookmarkName,
        message: Message<AtBookmark>,
    ) -> Result<(), EventError> {
        let bookmark_mailbox = self.bookmark_mailbox(bookmark)?;
        bookmark_mailbox.tell_domain(message);
        Ok(())
    }

    fn bookmark_mailbox(&mut self, bookmark: &BookmarkName) -> Result<BookmarkMailbox, EventError> {
        if let Some(mb) = self.bookmarks.get(bookmark) {
            return Ok(mb.clone());
        }
        // The bookmark actor needs both the reducer pipeline and the
        // chronicle handle — both live on the brain's `BookmarkEntry`,
        // but `brain_entry` returns the *active* entry. For a specific
        // bookmark we want its specific chronicle.
        let entry = self.canons.brain_entry(&self.brain)?;
        let chronicle = self.canons.bookmark_chronicle(&self.brain, bookmark)?;
        let entry = BookmarkEntry {
            pipeline: entry.pipeline,
            chronicle,
        };
        let mb = BookmarkActor::spawn(
            self.scope.config().clone(),
            self.brain.clone(),
            bookmark.clone(),
            entry,
        );
        self.bookmarks.insert(bookmark.clone(), mb.clone());
        Ok(mb)
    }
}
