//! `SystemLogActor` — singleton that appends events to the system event
//! log, then emits a projection apply message addressed to the system
//! projection actor.
//!
//! Stateless across messages: the scope on each `LogAppend` opens the
//! host DB. Receives the full `SystemMessage`; handles the log variants
//! and no-ops the rest.

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub struct SystemLogMailbox {
    tx: mpsc::UnboundedSender<SystemMessage>,
}

impl SystemLogMailbox {
    pub fn open() -> (Self, SystemLogInbox) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, SystemLogInbox { rx })
    }

    pub fn tell(&self, message: SystemMessage) {
        if let Err(error) = self.tx.send(message) {
            tracing::warn!(error = %error, "system log: receiver closed; message dropped");
        }
    }
}

pub struct SystemLogInbox {
    rx: mpsc::UnboundedReceiver<SystemMessage>,
}

impl SystemLogInbox {
    pub async fn recv(&mut self) -> Option<SystemMessage> {
        self.rx.recv().await
    }
}

pub struct SystemLogActor {
    mailbox: Mailbox,
}

impl SystemLogActor {
    pub fn spawn(inbox: SystemLogInbox, mailbox: Mailbox) {
        tokio::spawn(Self { mailbox }.run(inbox));
    }

    async fn run(self, mut inbox: SystemLogInbox) {
        while let Some(message) = inbox.recv().await {
            match message {
                SystemMessage::LogAppend(append) => {
                    if let Err(error) = self.append(append).await {
                        tracing::error!(?error, "system log: append failed");
                    }
                }
                SystemMessage::LogReset(_) => {
                    // Durable record — no-op for now.
                }
                _ => {}
            }
        }
    }

    async fn append(&self, append: AppendSystemLog) -> Result<(), EventError> {
        let AppendSystemLog { scope, event } = append;
        let host_db = HostDb::open(&scope).await?;
        let stored = EventLog::new(&host_db).append(&event)?;
        self.mailbox.tell(SystemMessage::from(
            ApplySystemProjection::builder()
                .scope(scope)
                .stored(stored)
                .build(),
        ));
        Ok(())
    }
}
