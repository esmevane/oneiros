//! Lifecycle operations — workflow orchestrations that compose domain services.
//!
//! These are NOT standalone domains with repos. They compose other domains'
//! services into higher-level operations: dream, introspect, reflect, sense, sleep.
//! Each produces lifecycle events and returns composed results.

pub mod errors;
pub mod model;
pub mod requests;
pub mod responses;
pub mod service;
pub mod features;

pub use features::projections::PROJECTIONS;
