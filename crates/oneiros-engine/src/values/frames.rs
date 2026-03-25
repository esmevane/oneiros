use crate::*;

/// A single frame — a set of independent projections at one dependency level.
#[derive(Clone)]
pub struct Frame {
    pub projections: Vec<Projection>,
}

impl Frame {
    pub fn new(projections: &[Projection]) -> Self {
        Self {
            projections: projections.to_vec(),
        }
    }
}

/// A set of ordered frames with a database connection.
///
/// Provides direct projection operations: migrate, apply, reset, replay.
/// Used by the engine for setup and replay. The FrameRunner wraps a
/// clone for async channel-based consumption.
#[derive(Clone)]
pub struct Frames {
    pub contents: Vec<Frame>,
}

impl Frames {
    pub fn new(frames: &[Frame]) -> Self {
        Self {
            contents: frames.to_vec(),
        }
    }
}
