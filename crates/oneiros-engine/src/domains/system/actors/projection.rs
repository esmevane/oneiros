//! `SystemProjectionActor` — singleton that applies system-tier
//! projections.
//!
//! Receives the full `SystemMessage`; handles the projection variants
//! and no-ops the rest.

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub(crate) struct SystemProjectionMailbox {
    tx: mpsc::UnboundedSender<SystemMessage>,
}

impl SystemProjectionMailbox {
    pub(crate) fn open() -> (Self, SystemProjectionInbox) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, SystemProjectionInbox { rx })
    }

    pub(crate) fn tell(&self, message: SystemMessage) {
        if let Err(error) = self.tx.send(message) {
            tracing::warn!(error = %error, "system projection: receiver closed; message dropped");
        }
    }
}

pub(crate) struct SystemProjectionInbox {
    rx: mpsc::UnboundedReceiver<SystemMessage>,
}

impl SystemProjectionInbox {
    pub(crate) async fn recv(&mut self) -> Option<SystemMessage> {
        self.rx.recv().await
    }
}

pub(crate) struct SystemProjectionActor;

impl SystemProjectionActor {
    pub(crate) fn spawn(inbox: SystemProjectionInbox) {
        tokio::spawn(Self.run(inbox));
    }

    async fn run(self, mut inbox: SystemProjectionInbox) {
        while let Some(message) = inbox.recv().await {
            match message {
                SystemMessage::ProjectionMigrate(migrate) => {
                    if let Err(error) = self.migrate(&migrate.scope).await {
                        tracing::error!(?error, "system projection: migrate failed");
                    }
                }
                SystemMessage::ProjectionApply(apply) => {
                    if let Err(error) = self.apply(&apply.scope, &apply.stored).await {
                        tracing::error!(?error, "system projection: apply failed");
                    }
                }
                SystemMessage::ProjectionReset(reset) => {
                    if let Err(error) = self.reset(&reset.scope).await {
                        tracing::error!(?error, "system projection: reset failed");
                    }
                }
                _ => {}
            }
        }
    }

    async fn migrate(&self, scope: &Scope<AtHost>) -> Result<(), EventError> {
        let host_db = HostDb::open(scope).await?;
        Projections::system().migrate(&host_db)?;
        Ok(())
    }

    async fn apply(&self, scope: &Scope<AtHost>, stored: &StoredEvent) -> Result<(), EventError> {
        let host_db = HostDb::open(scope).await?;
        let projections = Projections::system();
        projections.migrate(&host_db)?;
        projections.apply(&host_db, stored)?;
        Ok(())
    }

    async fn reset(&self, scope: &Scope<AtHost>) -> Result<(), EventError> {
        let host_db = HostDb::open(scope).await?;
        let projections = Projections::system();
        projections.reset(&host_db)?;
        projections.migrate(&host_db)?;
        Ok(())
    }
}
