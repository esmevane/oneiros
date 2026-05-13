//! `BookmarkChronicleActor` — singleton that records stored events into
//! the chronicle HAMT for any bookmark.
//!
//! First-time-seen `(project, bookmark)` pairs are caught up by walking
//! the project event log and recording each event. Subsequent `Record`
//! messages add one event each. Chronicle insertion is idempotent on
//! event id, so partial replay or duplicates are safe.
//!
//! Receives the full `BookmarkMessage`; handles the chronicle variants
//! and no-ops the rest.

use std::collections::HashSet;

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub(crate) struct BookmarkChronicleMailbox {
    tx: mpsc::UnboundedSender<BookmarkMessage>,
}

impl BookmarkChronicleMailbox {
    pub(crate) fn open() -> (Self, BookmarkChronicleInbox) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, BookmarkChronicleInbox { rx })
    }

    pub(crate) fn tell(&self, message: BookmarkMessage) {
        if let Err(error) = self.tx.send(message) {
            tracing::warn!(error = %error, "bookmark chronicle: receiver closed; message dropped");
        }
    }
}

pub(crate) struct BookmarkChronicleInbox {
    rx: mpsc::UnboundedReceiver<BookmarkMessage>,
}

impl BookmarkChronicleInbox {
    pub(crate) async fn recv(&mut self) -> Option<BookmarkMessage> {
        self.rx.recv().await
    }
}

pub(crate) struct BookmarkChronicleActor {
    canons: CanonIndex,
    caught_up: HashSet<(ProjectName, BookmarkName)>,
}

impl BookmarkChronicleActor {
    pub(crate) fn spawn(inbox: BookmarkChronicleInbox, canons: CanonIndex) {
        tokio::spawn(
            Self {
                canons,
                caught_up: HashSet::new(),
            }
            .run(inbox),
        );
    }

    async fn run(mut self, mut inbox: BookmarkChronicleInbox) {
        while let Some(message) = inbox.recv().await {
            match message {
                BookmarkMessage::ChronicleRecord(record) => {
                    if let Err(error) = self.handle_record(record).await {
                        tracing::error!(?error, "bookmark chronicle: record failed");
                    }
                }
                BookmarkMessage::ChronicleReset(reset) => {
                    if let Err(error) = self.replay(&reset.scope).await {
                        tracing::error!(?error, "bookmark chronicle: reset failed");
                    }
                }
                _ => {}
            }
        }
    }

    async fn handle_record(&mut self, record: RecordBookmarkChronicle) -> Result<(), EventError> {
        let key = (
            record.scope.project().name.clone(),
            record.scope.bookmark().name.clone(),
        );
        if !self.caught_up.contains(&key) {
            self.replay(&record.scope).await?;
            self.caught_up.insert(key);
        }
        self.record_one(&record.scope, &record.stored).await
    }

    async fn replay(&self, scope: &Scope<AtBookmark>) -> Result<(), EventError> {
        let events = {
            let bookmark_db = BookmarkDb::open(scope).await?;
            EventLog::attached(&bookmark_db).load_all()?
        };

        if events.is_empty() {
            return Ok(());
        }

        let chronicle = self.chronicle_for(&scope.project().name, &scope.bookmark().name)?;
        let host_db = HostDb::open(scope).await?;
        let store = ChronicleStore::new(&host_db);
        store.migrate()?;
        for event in &events {
            chronicle.record(event, &store.resolver(), &store.writer())?;
        }
        Ok(())
    }

    async fn record_one(
        &self,
        scope: &Scope<AtBookmark>,
        stored: &StoredEvent,
    ) -> Result<(), EventError> {
        let chronicle = self.chronicle_for(&scope.project().name, &scope.bookmark().name)?;
        let host_db = HostDb::open(scope).await?;
        let store = ChronicleStore::new(&host_db);
        store.migrate()?;
        chronicle.record(stored, &store.resolver(), &store.writer())?;
        Ok(())
    }

    fn chronicle_for(
        &self,
        project: &ProjectName,
        bookmark: &BookmarkName,
    ) -> Result<Chronicle, EventError> {
        self.canons.bookmark_chronicle(project, bookmark)
    }
}
