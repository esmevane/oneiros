//! `HostProjectionActor` — singleton that applies host-tier
//! projections.
//!
//! Receives the full `HostMessage`; handles the projection variants
//! and no-ops the rest.

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub(crate) struct HostProjectionMailbox {
    tx: mpsc::UnboundedSender<HostMessage>,
}

impl HostProjectionMailbox {
    pub(crate) fn open() -> (Self, HostProjectionInbox) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, HostProjectionInbox { rx })
    }

    pub(crate) fn tell(&self, message: HostMessage) {
        if let Err(error) = self.tx.send(message) {
            tracing::warn!(error = %error, "host projection: receiver closed; message dropped");
        }
    }
}

pub(crate) struct HostProjectionInbox {
    rx: mpsc::UnboundedReceiver<HostMessage>,
}

impl HostProjectionInbox {
    pub(crate) async fn recv(&mut self) -> Option<HostMessage> {
        self.rx.recv().await
    }
}

pub(crate) struct HostProjectionActor;

impl HostProjectionActor {
    pub(crate) fn spawn(inbox: HostProjectionInbox) {
        tokio::spawn(Self.run(inbox));
    }

    async fn run(self, mut inbox: HostProjectionInbox) {
        while let Some(message) = inbox.recv().await {
            match message {
                HostMessage::ProjectionMigrate(migrate) => {
                    if let Err(error) = self.migrate(&migrate.scope).await {
                        tracing::error!(?error, "host projection: migrate failed");
                    }
                }
                HostMessage::ProjectionApply(apply) => {
                    if let Err(error) = self.apply(&apply.scope, &apply.stored).await {
                        tracing::error!(?error, "host projection: apply failed");
                    }
                }
                HostMessage::ProjectionReset(reset) => {
                    if let Err(error) = self.reset(&reset.scope).await {
                        tracing::error!(?error, "host projection: reset failed");
                    }
                }
                _ => {}
            }
        }
    }

    async fn migrate(&self, scope: &Scope<AtHost>) -> Result<(), EventError> {
        let host_db = HostDb::open(scope).await?;
        Projections::host().migrate(&host_db)?;
        Ok(())
    }

    async fn apply(&self, scope: &Scope<AtHost>, stored: &StoredEvent) -> Result<(), EventError> {
        let host_db = HostDb::open(scope).await?;
        let projections = Projections::host();
        projections.migrate(&host_db)?;
        projections.apply(&host_db, stored)?;
        Ok(())
    }

    async fn reset(&self, scope: &Scope<AtHost>) -> Result<(), EventError> {
        let host_db = HostDb::open(scope).await?;
        let projections = Projections::host();
        projections.reset(&host_db)?;
        projections.migrate(&host_db)?;
        Ok(())
    }
}
