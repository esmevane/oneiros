//! Lifecycle operations — workflow orchestrations that compose domain services.
//!
//! These are NOT standalone domains with repos. They compose other domains'
//! services into higher-level operations: dream, introspect, reflect, sense, sleep.
//! Each produces lifecycle events and returns composed results.

mod client;
mod features;
mod model;
mod protocol;
mod service;

pub use client::LifecycleClient;
pub use features::mcp as lifecycle_mcp;
pub use features::LifecycleRouter;
pub use model::{CognitiveContext, LifecycleMarker};
pub use protocol::{
    LifecycleError, LifecycleEvent, LifecycleEvents, LifecycleRequest, LifecycleResponse,
    SensedEvent,
};
pub use service::LifecycleService;
