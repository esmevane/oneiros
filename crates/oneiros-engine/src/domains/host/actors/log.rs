//! `HostLogActor` — singleton that appends events to the host event
//! log, then emits a projection apply message addressed to the system
//! projection actor.
//!
//! Stateless across messages: the scope on each `LogAppend` opens the
//! host DB. Receives the full `HostMessage`; handles the log variants
//! and no-ops the rest.

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub(crate) struct HostLogMailbox {
    tx: mpsc::UnboundedSender<HostMessage>,
}

impl HostLogMailbox {
    pub(crate) fn open() -> (Self, HostLogInbox) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, HostLogInbox { rx })
    }

    pub(crate) fn tell(&self, message: HostMessage) {
        if let Err(error) = self.tx.send(message) {
            tracing::warn!(error = %error, "host log: receiver closed; message dropped");
        }
    }
}

pub(crate) struct HostLogInbox {
    rx: mpsc::UnboundedReceiver<HostMessage>,
}

impl HostLogInbox {
    pub(crate) async fn recv(&mut self) -> Option<HostMessage> {
        self.rx.recv().await
    }
}

pub(crate) struct HostLogActor {
    mailbox: Mailbox,
}

impl HostLogActor {
    pub(crate) fn spawn(inbox: HostLogInbox, mailbox: Mailbox) {
        tokio::spawn(Self { mailbox }.run(inbox));
    }

    async fn run(self, mut inbox: HostLogInbox) {
        while let Some(message) = inbox.recv().await {
            if let HostMessage::LogAppend(append) = message
                && let Err(error) = self.append(append).await
            {
                tracing::error!(?error, "host log: append failed");
            }
        }
    }

    async fn append(&self, append: AppendHostLog) -> Result<(), EventError> {
        let AppendHostLog { scope, event } = append;
        let host_db = HostDb::open(&scope).await?;
        let stored = EventLog::new(&host_db).append(&event)?;
        self.mailbox.tell(HostMessage::from(
            ApplyHostProjection::builder()
                .scope(scope)
                .stored(stored)
                .build(),
        ));
        Ok(())
    }
}
