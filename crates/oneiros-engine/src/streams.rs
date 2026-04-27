//! Streams — the surrogate `emit` delegates to.
//!
//! A `ProjectStream` owns the pieces that used to be inlined in
//! `ProjectContext::emit`: the wake channel (broadcast), the synchronous
//! subscribers (projections, chronicle), and enough config to open the
//! connections each needs.
//!
//! Publishing an event is:
//! 1. Append to the event log (truth).
//! 2. Synchronously fan out to each subscriber in order.
//! 3. Fire the wake (best-effort).
//!
//! Subscribers are sync callees for this spike — the whole publish call
//! completes before returning, matching today's emit semantics exactly.

use tokio::sync::broadcast;

use crate::*;

/// Something that reacts to an appended event.
///
/// Synchronous for this spike — iteration in `publish` waits for each
/// subscriber before moving to the next. Same behavior as today's
/// inline emit.
pub trait Subscriber: Send + Sync {
    fn name(&self) -> &'static str;
    fn on_event(&self, stored: &StoredEvent) -> Result<(), EventError>;
}

/// A project-scoped stream — per (brain, bookmark) for this spike.
///
/// Holds the pieces `emit` used to touch inline.
#[derive(Clone)]
pub struct ProjectStream {
    config: Config,
    wake: broadcast::Sender<StoredEvent>,
    subscribers: Vec<std::sync::Arc<dyn Subscriber>>,
}

impl ProjectStream {
    pub fn new(
        config: Config,
        wake: broadcast::Sender<StoredEvent>,
        subscribers: Vec<std::sync::Arc<dyn Subscriber>>,
    ) -> Self {
        Self {
            config,
            wake,
            subscribers,
        }
    }

    /// Subscribe to the wake channel — external listeners (e.g. SSE).
    pub fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.wake.subscribe()
    }

    /// Wait until all subscribers have caught up to the current log head.
    ///
    /// While subscribers fan out synchronously inside `publish`, this is
    /// a no-op — cursors are always at head when control returns to the
    /// caller. Ceremonies that need read-your-own-writes call this
    /// before querying projected state, so the code reads correctly and
    /// the call site survives the eventual flip to async subscribers.
    pub async fn wait_for_head(&self) -> Result<(), EventError> {
        Ok(())
    }

    /// Append an event, fan out to subscribers, fire the wake.
    pub fn publish(&self, event: impl Into<Events>) -> Result<StoredEvent, EventError> {
        let db = self.config.bookmark_conn()?;
        let new_event = NewEvent::builder().data(event).build();
        let stored = EventLog::attached(&db).append(&new_event)?;

        for subscriber in &self.subscribers {
            subscriber.on_event(&stored)?;
        }

        let _ = self.wake.send(stored.clone());
        Ok(stored)
    }
}

/// Subscriber that applies brain projections (tables + reducers +
/// pressure sync). Wraps today's `Projections::apply_brain`.
pub struct ProjectionSubscriber {
    projections: Projections<BrainCanon>,
    config: Config,
}

impl ProjectionSubscriber {
    pub fn new(projections: Projections<BrainCanon>, config: Config) -> Self {
        Self {
            projections,
            config,
        }
    }
}

impl Subscriber for ProjectionSubscriber {
    fn name(&self) -> &'static str {
        "projections"
    }

    fn on_event(&self, stored: &StoredEvent) -> Result<(), EventError> {
        let db = self.config.bookmark_conn()?;
        self.projections.apply_brain(&db, stored)
    }
}

/// A host-scoped stream — one per running service.
///
/// Simpler than `ProjectStream`: no chronicle, no wake (system events
/// don't broadcast today). One canonical subscriber: the system
/// projections.
#[derive(Clone)]
pub struct SystemStream {
    config: Config,
    subscribers: Vec<std::sync::Arc<dyn Subscriber>>,
}

impl SystemStream {
    pub fn new(config: Config, subscribers: Vec<std::sync::Arc<dyn Subscriber>>) -> Self {
        Self {
            config,
            subscribers,
        }
    }

    /// Wait until subscribers have caught up to the current log head.
    /// No-op while fan-out is synchronous; earns teeth when subscribers
    /// move to their own tasks.
    pub async fn wait_for_head(&self) -> Result<(), EventError> {
        Ok(())
    }

    /// Append a system event, fan out to subscribers.
    pub fn publish(&self, event: impl Into<Events>) -> Result<StoredEvent, EventError> {
        let db = self.config.system_db()?;
        let new_event = NewEvent::builder().data(event).build();
        let stored = EventLog::new(&db).append(&new_event)?;

        for subscriber in &self.subscribers {
            subscriber.on_event(&stored)?;
        }

        Ok(stored)
    }
}

/// Subscriber that applies system-level projections. Wraps today's
/// `Projections::<SystemCanon>::apply`.
pub struct SystemProjectionSubscriber {
    projections: Projections<SystemCanon>,
    config: Config,
}

impl SystemProjectionSubscriber {
    pub fn new(projections: Projections<SystemCanon>, config: Config) -> Self {
        Self {
            projections,
            config,
        }
    }
}

impl Subscriber for SystemProjectionSubscriber {
    fn name(&self) -> &'static str {
        "system-projections"
    }

    fn on_event(&self, stored: &StoredEvent) -> Result<(), EventError> {
        let db = self.config.system_db()?;
        self.projections.apply(&db, stored)
    }
}

/// Subscriber that records the event in the brain's chronicle (HAMT
/// root in the system DB). Wraps today's `Chronicle::record`.
pub struct ChronicleSubscriber {
    chronicle: Chronicle,
    config: Config,
}

impl ChronicleSubscriber {
    pub fn new(chronicle: Chronicle, config: Config) -> Self {
        Self { chronicle, config }
    }
}

impl Subscriber for ChronicleSubscriber {
    fn name(&self) -> &'static str {
        "chronicle"
    }

    fn on_event(&self, stored: &StoredEvent) -> Result<(), EventError> {
        let system_db = self.config.system_db()?;
        let store = ChronicleStore::new(&system_db);
        store.migrate()?;
        self.chronicle
            .record(stored, &store.resolver(), &store.writer())
    }
}
