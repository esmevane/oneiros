//! Projector — subscribes to event broadcasts and materializes read models.
//!
//! The projector is the active component that turns an event stream into
//! queryable state. It subscribes to the EventBus broadcast, applies each
//! event to its projections, and chronicles it.
//!
//! Projectors process events asynchronously on their own spawned task.
//! Reads that need fresh state should use eventual-consistency retries.

use std::sync::{Arc, Mutex};

use tokio::sync::broadcast;

use crate::*;

/// A running projector — spawned as a tokio task, processes events
/// from the broadcast channel.
pub(crate) struct Projector;

impl Projector {
    /// Spawn a brain-level projector.
    ///
    /// Subscribes to the broadcast, applies each event through the
    /// brain projection pipeline (frames + reducers + pressure sync),
    /// and records to the chronicle.
    pub(crate) fn spawn_brain(
        db: Arc<Mutex<rusqlite::Connection>>,
        projections: Projections<BrainCanon>,
        chronicle: Chronicle,
        broadcast: &broadcast::Sender<StoredEvent>,
    ) {
        let mut receiver = broadcast.subscribe();

        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                let result = (|| -> Result<(), EventError> {
                    let conn = db
                        .lock()
                        .map_err(|e| EventError::Lock(e.to_string()))?;

                    projections.apply_brain(&conn, &event)?;

                    let chronicle_store = ChronicleStore::new(&conn);
                    chronicle_store.migrate()?;
                    chronicle.record(
                        &event,
                        &chronicle_store.resolver(),
                        &chronicle_store.writer(),
                    )?;

                    Ok(())
                })();

                if let Err(e) = result {
                    eprintln!("projector: brain projection failed for event {}: {e}", event.id);
                }
            }
        });
    }

    /// Spawn a system-level projector.
    ///
    /// Subscribes to the broadcast and applies each event through
    /// the system projection pipeline.
    pub(crate) fn spawn_system(
        db: Arc<Mutex<rusqlite::Connection>>,
        projections: Projections<SystemCanon>,
        broadcast: &broadcast::Sender<StoredEvent>,
    ) {
        let mut receiver = broadcast.subscribe();

        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                let result = (|| -> Result<(), EventError> {
                    let conn = db
                        .lock()
                        .map_err(|e| EventError::Lock(e.to_string()))?;

                    projections.apply(&conn, &event)?;

                    Ok(())
                })();

                if let Err(e) = result {
                    eprintln!("projector: system projection failed for event {}: {e}", event.id);
                }
            }
        });
    }
}
