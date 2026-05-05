//! `ProjectImportActor` — singleton that handles foreign event ingestion
//! for any brain. Insert-or-ignore into events.db keyed on event id,
//! then emit the same downstream messages as a local append.
//!
//! Stateless across messages: the scope on each `ImportEvent` opens
//! the correct events.db. Receives the full `ProjectMessage`; handles
//! the import variants and no-ops the rest.

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub struct ProjectImportMailbox {
    tx: mpsc::UnboundedSender<ProjectMessage>,
}

impl ProjectImportMailbox {
    pub fn open() -> (Self, ProjectImportInbox) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, ProjectImportInbox { rx })
    }

    pub fn tell(&self, message: ProjectMessage) {
        if let Err(error) = self.tx.send(message) {
            tracing::warn!(error = %error, "project import: receiver closed; message dropped");
        }
    }
}

pub struct ProjectImportInbox {
    rx: mpsc::UnboundedReceiver<ProjectMessage>,
}

impl ProjectImportInbox {
    pub async fn recv(&mut self) -> Option<ProjectMessage> {
        self.rx.recv().await
    }
}

pub struct ProjectImportActor {
    mailbox: Mailbox,
}

impl ProjectImportActor {
    pub fn spawn(inbox: ProjectImportInbox, mailbox: Mailbox) {
        tokio::spawn(Self { mailbox }.run(inbox));
    }

    async fn run(self, mut inbox: ProjectImportInbox) {
        while let Some(message) = inbox.recv().await {
            match message {
                ProjectMessage::ImportEvent(import) => {
                    if let Err(error) = self.import(import).await {
                        tracing::error!(?error, "project import: ingest failed");
                    }
                }
                ProjectMessage::ImportReset(_) => {
                    // No actor-local state to reset.
                }
                _ => {}
            }
        }
    }

    async fn import(&self, import: ImportProjectEvent) -> Result<(), EventError> {
        let ImportProjectEvent { scope, stored } = import;
        let events_db = EventsDb::open(&scope).await?;
        EventLog::new(&events_db).init()?;
        EventLog::new(&events_db).import(&stored)?;
        drop(events_db);

        self.mailbox.tell(BookmarkMessage::from(
            ApplyBookmarkProjection::builder()
                .scope(scope.clone())
                .stored(stored.clone())
                .build(),
        ));
        self.mailbox.tell(BookmarkMessage::from(
            RecordBookmarkChronicle::builder()
                .scope(scope)
                .stored(stored)
                .build(),
        ));

        Ok(())
    }
}
