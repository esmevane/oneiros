//! EventBus — append, dispatch, broadcast.
//!
//! The bus owns event persistence and notification. It does NOT run
//! projections — that's the Frames' job. The bus has three steps:
//!
//! 1. **Append** — persist to the EventLog (durable fact)
//! 2. **Dispatch** — send to Frames via mpsc (guaranteed delivery)
//! 3. **Broadcast** — send to external observers via broadcast (lossy ok)
//!
//! Import bypasses the bus entirely — it writes to the EventLog directly
//! and then triggers a replay through the Frames.

use std::sync::{Arc, Mutex, MutexGuard};

use tokio::sync::{broadcast, mpsc, oneshot};

use crate::*;

/// A dispatched event — carries the event and an acknowledgment channel.
pub(crate) struct Dispatch {
    pub(crate) event: StoredEvent,
    pub(crate) ack: oneshot::Sender<()>,
}

/// The event bus — append, dispatch, broadcast.
#[derive(Clone)]
pub(crate) struct EventBus {
    db: Arc<Mutex<rusqlite::Connection>>,
    dispatch: mpsc::UnboundedSender<Dispatch>,
    broadcast: broadcast::Sender<StoredEvent>,
}

/// Acquire the database lock, converting PoisonError to EventError.
fn lock(
    db: &Mutex<rusqlite::Connection>,
) -> Result<MutexGuard<'_, rusqlite::Connection>, EventError> {
    db.lock().map_err(|e| EventError::Lock(e.to_string()))
}

impl EventBus {
    /// Create a new bus and spawn its consumer task.
    ///
    /// The consumer owns the projection pipeline: for each dispatched
    /// event it applies projections, chronicles the event, and
    /// broadcasts to SSE subscribers. The caller receives an `EventBus`
    /// handle for publishing events and subscribing to broadcasts.
    ///
    /// The caller is responsible for running EventLog and projection
    /// migrations before publishing any events.
    pub(crate) fn spawn(
        db: Arc<Mutex<rusqlite::Connection>>,
        projections: Projections<BrainCanon>,
        chronicle: Chronicle,
    ) -> Self {
        let (bus, receiver) = Self::channels(db);
        let consumer_db = bus.db.clone();
        let broadcast = bus.broadcast.clone();

        tokio::spawn(Self::consumer(
            receiver,
            consumer_db,
            projections,
            chronicle,
            broadcast,
        ));

        bus
    }

    /// The consumer loop — processes dispatched events sequentially.
    ///
    /// Each event is projected, chronicled, and broadcast. The ack
    /// channel signals the publisher that projection is complete, so
    /// read-after-write queries see consistent state.
    async fn consumer(
        mut receiver: mpsc::UnboundedReceiver<Dispatch>,
        db: Arc<Mutex<rusqlite::Connection>>,
        projections: Projections<BrainCanon>,
        chronicle: Chronicle,
        broadcast: broadcast::Sender<StoredEvent>,
    ) {
        while let Some(Dispatch { event, ack }) = receiver.recv().await {
            let result = (|| -> Result<(), EventError> {
                let conn = lock(&db)?;

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
                eprintln!("projection failed for event {}: {e}", event.id);
            }

            let _ = broadcast.send(event);
            let _ = ack.send(());
        }
    }

    /// Create a new bus with raw channels. Returns the bus and the
    /// receiver for manual consumer setup.
    ///
    /// Prefer `spawn()` which creates the bus and starts the consumer.
    /// Use this only when you need custom consumer logic.
    fn channels(db: Arc<Mutex<rusqlite::Connection>>) -> (Self, mpsc::UnboundedReceiver<Dispatch>) {
        let (dispatch, receiver) = mpsc::unbounded_channel();
        let (broadcast, _) = broadcast::channel(256);

        let bus = Self {
            db,
            dispatch,
            broadcast,
        };

        (bus, receiver)
    }

    /// Publish an event: append to log, dispatch to consumer, await projection.
    ///
    /// Returns after the event has been persisted AND projected.
    /// The consumer handles projection, chronicling, and broadcasting.
    pub(crate) async fn publish(&self, event: NewEvent) -> Result<StoredEvent, EventError> {
        let stored = {
            let conn = lock(&self.db)?;
            EventLog::new(&conn).append(&event)?
        };

        let (ack_tx, ack_rx) = oneshot::channel();
        let _ = self.dispatch.send(Dispatch {
            event: stored.clone(),
            ack: ack_tx,
        });

        let _ = ack_rx.await;

        Ok(stored)
    }

    /// The broadcast sender for SSE subscribers.
    pub(crate) fn broadcast(&self) -> &broadcast::Sender<StoredEvent> {
        &self.broadcast
    }

    /// Emit an event with a given source: construct + publish.
    pub(crate) async fn emit(
        &self,
        event: impl Into<Events>,
        source: Source,
    ) -> Result<StoredEvent, EventError> {
        let new_event = NewEvent {
            data: event.into(),
            source,
        };
        self.publish(new_event).await
    }

    /// Subscribe to the broadcast channel (for SSE, dashboard, peers).
    pub(crate) fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.broadcast.subscribe()
    }

    /// Execute a read operation against the database.
    pub(crate) fn with_db<T>(&self, f: impl FnOnce(&rusqlite::Connection) -> T) -> Result<T, EventError> {
        let conn = lock(&self.db)?;
        Ok(f(&conn))
    }

    /// Access the shared database handle (for Frames construction).
    pub(crate) fn db(&self) -> Arc<Mutex<rusqlite::Connection>> {
        self.db.clone()
    }

    /// Load all events from the log (for export, replay source).
    pub(crate) fn load_events(&self) -> Result<Vec<StoredEvent>, EventError> {
        let conn = lock(&self.db)?;
        EventLog::new(&conn).load_all()
    }

    /// Dispatch and broadcast a stored event (for replay).
    ///
    /// Unlike publish, this does NOT append — the event already exists
    /// in the log. It just sends it through the dispatch and broadcast
    /// channels so Frames and external observers see it.
    pub(crate) async fn redispatch(&self, event: StoredEvent) {
        let (ack_tx, ack_rx) = oneshot::channel();
        let _ = self.dispatch.send(Dispatch {
            event: event.clone(),
            ack: ack_tx,
        });
        let _ = self.broadcast.send(event);
        let _ = ack_rx.await;
    }

    /// Import an event to the log without dispatching or broadcasting.
    ///
    /// For bulk import. After import, call replay to dispatch all events
    /// through the projection pipeline.
    pub(crate) fn import(&self, event: &crate::StoredEvent) -> Result<(), EventError> {
        let conn = lock(&self.db)?;
        EventLog::new(&conn).import(event)
    }
}
