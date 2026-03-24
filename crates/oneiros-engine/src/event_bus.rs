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

use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use tokio::sync::{broadcast, mpsc, oneshot};

use crate::event::EventError;
use crate::event_log::EventLog;
use crate::events::Events;
use crate::{NewEvent, Source, StoredEvent};

/// A dispatched event — carries the event and an acknowledgment channel.
pub struct Dispatch {
    pub event: StoredEvent,
    pub ack: oneshot::Sender<()>,
}

/// The event bus — append, dispatch, broadcast.
#[derive(Clone)]
pub struct EventBus {
    db: Arc<Mutex<Connection>>,
    dispatch: mpsc::UnboundedSender<Dispatch>,
    broadcast: broadcast::Sender<StoredEvent>,
}

impl EventBus {
    /// Create a new bus. Returns the bus and an mpsc receiver for Frames.
    ///
    /// The caller is responsible for:
    /// 1. Running EventLog migrations
    /// 2. Running projection migrations (via Frames)
    /// 3. Spawning the Frames task with the returned receiver
    pub fn new(db: Arc<Mutex<Connection>>) -> (Self, mpsc::UnboundedReceiver<Dispatch>) {
        let (dispatch, receiver) = mpsc::unbounded_channel();
        let (broadcast, _) = broadcast::channel(256);

        let bus = Self {
            db,
            dispatch,
            broadcast,
        };

        (bus, receiver)
    }

    /// Publish an event: append + dispatch + broadcast.
    ///
    /// Returns after the event has been persisted AND projected.
    /// The dispatch channel carries a oneshot for acknowledgment —
    /// the Frames task signals when projection is complete.
    pub async fn publish(&self, event: NewEvent) -> Result<StoredEvent, EventError> {
        // 1. Append — persist to the event log
        let stored = {
            let conn = self.db.lock().expect("db lock");
            EventLog::new(&conn).append(&event)?
        };

        // 2. Dispatch — send to Frames with ack channel
        let (ack_tx, ack_rx) = oneshot::channel();
        let _ = self.dispatch.send(Dispatch {
            event: stored.clone(),
            ack: ack_tx,
        });

        // 3. Broadcast — send to external observers (lossy ok)
        let _ = self.broadcast.send(stored.clone());

        // 4. Wait for projection to complete
        let _ = ack_rx.await;

        Ok(stored)
    }

    /// Emit an event with a given source: construct + publish.
    pub async fn emit(
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
    pub fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.broadcast.subscribe()
    }

    /// Execute a read operation against the database.
    pub fn with_db<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        let conn = self.db.lock().expect("db lock");
        f(&conn)
    }

    /// Access the shared database handle (for Frames construction).
    pub fn db(&self) -> Arc<Mutex<Connection>> {
        self.db.clone()
    }

    /// Load all events from the log (for export, replay source).
    pub fn load_events(&self) -> Result<Vec<StoredEvent>, EventError> {
        let conn = self.db.lock().expect("db lock");
        EventLog::new(&conn).load_all()
    }

    /// Dispatch and broadcast a stored event (for replay).
    ///
    /// Unlike publish, this does NOT append — the event already exists
    /// in the log. It just sends it through the dispatch and broadcast
    /// channels so Frames and external observers see it.
    pub async fn redispatch(&self, event: StoredEvent) {
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
    pub fn import(&self, event: &crate::ImportEvent) -> Result<(), EventError> {
        let conn = self.db.lock().expect("db lock");
        EventLog::new(&conn).import(event)
    }
}
