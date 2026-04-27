//! Continuity operations — workflow orchestrations that compose domain services.
//!
//! These are NOT standalone domains with repos. They compose other domains'
//! services into higher-level operations: dream, introspect, reflect, sense, sleep.
//! Each produces continuity events and returns composed results.

mod client;
mod docs;
mod features;
mod presenter;
mod protocol;
mod service;
mod view;

pub use client::*;
pub use docs::*;
pub use features::*;
pub use presenter::*;
pub use protocol::*;
pub use service::*;
pub use view::*;
