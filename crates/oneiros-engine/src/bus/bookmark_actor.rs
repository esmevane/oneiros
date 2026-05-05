//! `BookmarkActor` — per-bookmark actor that owns both projection
//! application and chronicle recording.
//!
//! One actor per `(brain, bookmark)` pair. Spawned lazily by the
//! `ProjectActor` the first time it sees an event for a bookmark.
//!
//! On each `Stored` event the actor does two things in order:
//! 1. Apply brain projections to the bookmark DB.
//! 2. Record the event in the bookmark's chronicle (HAMT in the host
//!    system DB).
//!
//! Doing both in the same actor — per-event, sequential — means there's
//! no race between projection and chronicle for distribution callers.
//! Wait for the chronicle to settle and the projection has settled too.
//!
//! On initial spawn the actor catches up: walks the project's event log
//! and applies each event through projection + chronicle. Both
//! operations are idempotent (delete-then-insert projections; HAMT
//! insert by event id), so replaying the log is safe whether starting
//! empty or inheriting from a fork. A `Reset` lifecycle message
//! triggers a full rebuild — same shape, on demand.

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub struct BookmarkMailbox {
    tx: mpsc::UnboundedSender<LifecycleMessage<Message<AtBookmark>>>,
}

impl BookmarkMailbox {
    pub fn tell(&self, message: LifecycleMessage<Message<AtBookmark>>) {
        if let Err(err) = self.tx.send(message) {
            tracing::warn!(error = %err, "bookmark actor: receiver closed; message dropped");
        }
    }

    /// Convenience — wrap a domain message and send.
    pub fn tell_domain(&self, message: Message<AtBookmark>) {
        self.tell(LifecycleMessage::Domain(message));
    }

    /// Convenience — send a Reset signal.
    pub fn reset(&self) {
        self.tell(LifecycleMessage::Reset);
    }
}

pub struct BookmarkActor {
    config: Config,
    brain: BrainName,
    bookmark: BookmarkName,
    projections: Projections<BrainCanon>,
    chronicle: Chronicle,
}

impl BookmarkActor {
    /// Spawn a bookmark actor. The reducer pipeline from the entry is
    /// wired into the projection set; the chronicle handle is the
    /// shared `Arc<Mutex>` from `CanonIndex`.
    ///
    /// The actor catches up from history on its first run, before
    /// processing any incoming domain messages.
    pub fn spawn(
        config: Config,
        brain: BrainName,
        bookmark: BookmarkName,
        entry: BookmarkEntry,
    ) -> BookmarkMailbox {
        let (tx, rx) = mpsc::unbounded_channel();
        let actor = Self {
            config,
            brain,
            bookmark,
            projections: Projections::project_with_pipeline(entry.pipeline),
            chronicle: entry.chronicle,
        };
        tokio::spawn(actor.run(rx));
        BookmarkMailbox { tx }
    }

    async fn run(self, mut rx: mpsc::UnboundedReceiver<LifecycleMessage<Message<AtBookmark>>>) {
        if let Err(err) = self.catch_up().await {
            tracing::error!(
                brain = %self.brain,
                bookmark = %self.bookmark,
                ?err,
                "bookmark actor: initial catch-up failed",
            );
        }

        while let Some(message) = rx.recv().await {
            match message {
                LifecycleMessage::Domain(message) => {
                    if let Err(err) = self.handle(message).await {
                        tracing::error!(
                            brain = %self.brain,
                            bookmark = %self.bookmark,
                            ?err,
                            "bookmark actor: handle failed",
                        );
                    }
                }
                LifecycleMessage::Reset => {
                    if let Err(err) = self.catch_up().await {
                        tracing::error!(
                            brain = %self.brain,
                            bookmark = %self.bookmark,
                            ?err,
                            "bookmark actor: reset/catch-up failed",
                        );
                    }
                }
            }
        }
    }

    /// Replay the project's event log into this bookmark — wipes
    /// projection state, walks every event, applies each through
    /// projections AND records it in the chronicle. Used for both
    /// first-spawn catch-up and explicit `Reset`.
    async fn catch_up(&self) -> Result<(), EventError> {
        // Step 1: projections (and capture the events list while the
        // bookmark DB is open). Drop the connection before opening
        // the host DB — `EventLog<'_>` borrows the connection's
        // statement cache (RefCell), which isn't Send across awaits.
        let events = {
            let bookmark_db =
                BookmarkDb::open_with(&self.config.platform(), &self.brain, &self.bookmark).await?;
            self.projections.migrate(&bookmark_db)?;
            let log = EventLog::attached(&bookmark_db);
            self.projections.replay_brain(&bookmark_db, &log)?;
            log.load_all()?
        };

        if events.is_empty() {
            return Ok(());
        }

        // Step 2: chronicle catch-up — record every event. Idempotent
        // on event id.
        let host_db = HostDb::open_with(&self.config.platform()).await?;
        let store = ChronicleStore::new(&host_db);
        store.migrate()?;
        for event in &events {
            self.chronicle
                .record(event, &store.resolver(), &store.writer())?;
        }
        Ok(())
    }

    async fn handle(&self, message: Message<AtBookmark>) -> Result<(), EventError> {
        let stored = match message.event {
            Event::Stored(boxed) => *boxed,
            _ => return Ok(()),
        };

        // 1. Apply projection.
        let bookmark_db =
            BookmarkDb::open_with(&self.config.platform(), &self.brain, &self.bookmark).await?;
        self.projections.apply_brain(&bookmark_db, &stored)?;
        drop(bookmark_db);

        // 2. Record in chronicle. Sequential — by the time this
        //    returns, both projection and chronicle reflect the
        //    event. No race for downstream waiters.
        let host_db = HostDb::open_with(&self.config.platform()).await?;
        let store = ChronicleStore::new(&host_db);
        store.migrate()?;
        self.chronicle
            .record(&stored, &store.resolver(), &store.writer())?;

        Ok(())
    }
}
