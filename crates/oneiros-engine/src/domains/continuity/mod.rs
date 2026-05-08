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

pub(crate) use client::*;
pub(crate) use docs::*;
pub(crate) use features::*;
pub(crate) use presenter::*;
pub(crate) use protocol::*;
pub(crate) use service::*;
pub(crate) use view::*;
