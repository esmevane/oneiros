//! `ProjectLogActor` — singleton that owns the events.db append for any
//! brain. After appending, emits two follow-up messages addressed to
//! the bookmark projections and bookmark chronicle actors.
//!
//! Stateless across messages: the scope on each `LogAppend` opens the
//! correct events.db. Receives the full `ProjectMessage`; handles the
//! log variants and no-ops the rest.

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub(crate) struct ProjectLogMailbox {
    tx: mpsc::UnboundedSender<ProjectMessage>,
}

impl ProjectLogMailbox {
    pub(crate) fn open() -> (Self, ProjectLogInbox) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, ProjectLogInbox { rx })
    }

    pub(crate) fn tell(&self, message: ProjectMessage) {
        if let Err(error) = self.tx.send(message) {
            tracing::warn!(error = %error, "project log: receiver closed; message dropped");
        }
    }
}

pub(crate) struct ProjectLogInbox {
    rx: mpsc::UnboundedReceiver<ProjectMessage>,
}

impl ProjectLogInbox {
    pub(crate) async fn recv(&mut self) -> Option<ProjectMessage> {
        self.rx.recv().await
    }
}

pub(crate) struct ProjectLogActor {
    mailbox: Mailbox,
}

impl ProjectLogActor {
    pub(crate) fn spawn(inbox: ProjectLogInbox, mailbox: Mailbox) {
        tokio::spawn(Self { mailbox }.run(inbox));
    }

    async fn run(self, mut inbox: ProjectLogInbox) {
        while let Some(message) = inbox.recv().await {
            if let ProjectMessage::LogAppend(append) = message
                && let Err(error) = self.append(append).await
            {
                tracing::error!(?error, "project log: append failed");
            }
        }
    }

    async fn append(&self, append: AppendProjectLog) -> Result<(), EventError> {
        let AppendProjectLog { scope, event } = append;
        let events_db = EventsDb::open(&scope).await?;
        EventLog::new(&events_db).init()?;
        let stored = Box::new(EventLog::new(&events_db).append(&event)?);

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
