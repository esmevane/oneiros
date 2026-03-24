//! System context — system-scoped infrastructure.
//!
//! Carries the event bus for system-scoped domains (tenants, actors,
//! tickets, brains). System-scoped domain services receive this.

use rusqlite::Connection;
use tokio::sync::broadcast;

use crate::event::EventError;
use crate::event_bus::EventBus;
use crate::events::Events;
use crate::{Source, StoredEvent};

/// The system-scoped application context.
#[derive(Clone)]
pub struct SystemContext {
    bus: EventBus,
    source: Source,
}

impl SystemContext {
    pub fn new(bus: EventBus) -> Self {
        Self {
            bus,
            source: Source::default(),
        }
    }

    pub fn with_source(mut self, source: Source) -> Self {
        self.source = source;
        self
    }

    pub fn with_db<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        self.bus.with_db(f)
    }

    pub async fn emit(&self, event: impl Into<Events>) -> Result<StoredEvent, EventError> {
        self.bus.emit(event, self.source).await
    }

    pub fn subscribe(&self) -> broadcast::Receiver<StoredEvent> {
        self.bus.subscribe()
    }
}
