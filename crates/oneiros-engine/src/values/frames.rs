//! Frames — ordered projection pipeline.
//!
//! Frame definitions describe which projections run at each dependency
//! level. The FrameRunner consumes events from a channel and applies
//! them. The FrameSet provides direct access for replay operations.

use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use tokio::sync::mpsc;

use crate::StoredEvent;
use crate::event::EventError;
use crate::event_bus::Dispatch;
use crate::values::projection::Projection;

/// A single frame — a set of independent projections at one dependency level.
pub struct Frame {
    projections: &'static [Projection],
}

impl Frame {
    pub const fn new(projections: &'static [Projection]) -> Self {
        Self { projections }
    }

    pub fn migrate(&self, conn: &Connection) -> Result<(), EventError> {
        for projection in self.projections {
            (projection.migrate)(conn)?;
        }
        Ok(())
    }

    pub fn apply(&self, conn: &Connection, event: &StoredEvent) -> Result<(), EventError> {
        for projection in self.projections {
            (projection.apply)(conn, event)?;
        }
        Ok(())
    }

    pub fn reset(&self, conn: &Connection) -> Result<(), EventError> {
        for projection in self.projections {
            (projection.reset)(conn)?;
        }
        Ok(())
    }
}

/// A set of ordered frames with a database connection.
///
/// Provides direct projection operations: migrate, apply, reset, replay.
/// Used by the engine for setup and replay. The FrameRunner wraps a
/// clone for async channel-based consumption.
#[derive(Clone)]
pub struct Frames {
    frames: Arc<Vec<Frame>>,
    db: Arc<Mutex<Connection>>,
}

impl Frames {
    pub fn new(frames: Vec<Frame>, db: Arc<Mutex<Connection>>) -> Self {
        Self {
            frames: Arc::new(frames),
            db,
        }
    }

    /// Run all projection migrations.
    pub fn migrate(&self) -> Result<(), EventError> {
        let conn = self.db.lock().expect("db lock");
        for frame in self.frames.iter() {
            frame.migrate(&conn)?;
        }
        Ok(())
    }

    /// Apply a single event through all frames in order.
    pub fn apply(&self, event: &StoredEvent) -> Result<(), EventError> {
        let conn = self.db.lock().expect("db lock");
        for frame in self.frames.iter() {
            frame.apply(&conn, event)?;
        }
        Ok(())
    }

    /// Reset all projections across all frames.
    pub fn reset(&self) -> Result<(), EventError> {
        let conn = self.db.lock().expect("db lock");
        for frame in self.frames.iter() {
            frame.reset(&conn)?;
        }
        Ok(())
    }

    /// Replay a set of events through all frames (for import/rebuild).
    pub fn replay(&self, events: &[StoredEvent]) -> Result<usize, EventError> {
        self.reset()?;
        for event in events {
            self.apply(event)?;
        }
        Ok(events.len())
    }
}

/// The async frame runner — consumes events from a dispatch channel.
///
/// Spawned as a tokio task. Receives dispatched events from the bus,
/// applies them through the frame pipeline, and acknowledges completion
/// so the bus caller knows the read model is consistent.
pub struct FrameRunner {
    frames: Frames,
    receiver: mpsc::UnboundedReceiver<Dispatch>,
}

impl FrameRunner {
    pub fn new(frames: Frames, receiver: mpsc::UnboundedReceiver<Dispatch>) -> Self {
        Self { frames, receiver }
    }

    /// Run the projection loop — consume events, project, acknowledge.
    pub async fn run(mut self) {
        while let Some(dispatch) = self.receiver.recv().await {
            if let Err(e) = self.frames.apply(&dispatch.event) {
                eprintln!("projection error: {e}");
            }
            // Acknowledge completion — the bus caller can proceed
            let _ = dispatch.ack.send(());
        }
    }
}
