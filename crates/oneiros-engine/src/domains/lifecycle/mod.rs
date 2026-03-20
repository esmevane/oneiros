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

pub use client::*;
pub use features::*;
pub use model::*;
pub use protocol::*;
pub use service::*;
