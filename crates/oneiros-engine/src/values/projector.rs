//! Projector — materializes read models from the event stream.
//!
//! The projector owns the full projection lifecycle:
//! - Incremental: applies new events from the broadcast
//! - Rebuild: resets and replays for the active bookmark's event set
//!
//! All projection work runs on a single spawned task (incremental)
//! or the caller's task via the handle (rebuild, synchronized by mutex).

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use tokio::sync::broadcast;

use crate::*;

/// A handle to a running brain projector.
#[derive(Clone)]
pub(crate) struct ProjectorHandle {
    db: Arc<Mutex<rusqlite::Connection>>,
    projections: Projections<BrainCanon>,
    canons: CanonIndex,
    brain: BrainName,
    epoch: Arc<AtomicU64>,
}

impl ProjectorHandle {
    /// Rebuild projections from the active bookmark's event set.
    ///
    /// Runs synchronously on the caller's task. Acquires the DB
    /// mutex to serialize with the projector's event processing.
    /// Increments the epoch so the projector skips stale broadcasts.
    pub(crate) fn rebuild(&self) {
        self.epoch.fetch_add(1, Ordering::SeqCst);
        Projector::rebuild(&self.db, &self.projections, &self.canons, &self.brain);
    }
}

pub(crate) struct Projector;

impl Projector {
    /// Spawn a brain-level projector.
    pub(crate) fn spawn_brain(
        db: Arc<Mutex<rusqlite::Connection>>,
        projections: Projections<BrainCanon>,
        canons: CanonIndex,
        brain: BrainName,
        event_broadcast: &broadcast::Sender<StoredEvent>,
    ) -> ProjectorHandle {
        let event_db = db.clone();
        let event_projections = projections.clone();
        let event_canons = canons.clone();
        let event_brain = brain.clone();
        let epoch = Arc::new(AtomicU64::new(0));
        let event_epoch = epoch.clone();
        let mut events = event_broadcast.subscribe();

        tokio::spawn(async move {
            let mut my_epoch = 0u64;

            while let Ok(event) = events.recv().await {
                let current = event_epoch.load(Ordering::SeqCst);
                if current > my_epoch {
                    my_epoch = current;
                    while events.try_recv().is_ok() {}
                    continue;
                }

                Self::apply_event(
                    &event_db,
                    &event_projections,
                    &event_canons,
                    &event_brain,
                    &event,
                );
            }
        });

        ProjectorHandle {
            db,
            projections,
            canons,
            brain,
            epoch,
        }
    }

    fn apply_event(
        db: &Arc<Mutex<rusqlite::Connection>>,
        projections: &Projections<BrainCanon>,
        canons: &CanonIndex,
        brain: &BrainName,
        event: &StoredEvent,
    ) {
        let result = (|| -> Result<(), EventError> {
            let conn = db
                .lock()
                .map_err(|e| EventError::Lock(e.to_string()))?;

            projections.apply_brain(&conn, event)?;

            let chronicle = canons.chronicle(brain)?;
            let chronicle_store = ChronicleStore::new(&conn);
            chronicle_store.migrate()?;
            chronicle.record(
                event,
                &chronicle_store.resolver(),
                &chronicle_store.writer(),
            )?;

            Ok(())
        })();

        if let Err(e) = result {
            eprintln!("projector: apply failed for event {}: {e}", event.id);
        }
    }

    fn rebuild(
        db: &Arc<Mutex<rusqlite::Connection>>,
        projections: &Projections<BrainCanon>,
        canons: &CanonIndex,
        brain: &BrainName,
    ) {
        let result = (|| -> Result<(), EventError> {
            let conn = db
                .lock()
                .map_err(|e| EventError::Lock(e.to_string()))?;

            let chronicle = canons.chronicle(brain)?;
            let chronicle_store = ChronicleStore::new(&conn);
            chronicle_store.migrate()?;

            let root = chronicle.root()?;
            let visible_ids = Ledger::collect_all_ids(
                root.as_ref(),
                &chronicle_store.resolver(),
            );

            let all_events = EventLog::new(&conn).load_all()?;

            projections.reset(&conn)?;

            conn.execute_batch("BEGIN")?;

            let replay_result = (|| -> Result<(), EventError> {
                for event in &all_events {
                    if visible_ids.contains(&event.id.to_string()) {
                        projections.apply_frames(&conn, event)?;
                    }
                }
                Ok(())
            })();

            match replay_result {
                Ok(()) => conn.execute_batch("COMMIT")?,
                Err(e) => {
                    let _ = conn.execute_batch("ROLLBACK");
                    return Err(e);
                }
            }

            Ok(())
        })();

        if let Err(e) = result {
            eprintln!("projector: rebuild failed for brain {brain}: {e}");
        }
    }

    /// Spawn a system-level projector.
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
