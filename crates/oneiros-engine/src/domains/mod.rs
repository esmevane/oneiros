//! Domain modules — one domain ≈ one event type ≈ one service ≈ one feature set.
//!
//! Most domains follow this isomorphism exactly. Four are named exceptions:
//!
//! - **Bookmark** — host-scoped at the service boundary, not bookmark-scoped,
//!   because bookmarks define the scope that other domains operate within.
//! - **Search** — strangler at the service signature; will evolve with the
//!   lens work. Currently couples to multiple domain projections.
//! - **Follow** — handles foreign events (`BookmarkEvents`) rather than its
//!   own event type, because follows react to bookmark lifecycle.
//! - **Trail** — aggregates events from nearly every domain to produce
//!   cross-cutting activity views.

mod actor;
mod agent;
mod bookmark;
mod bridge;
mod chronicle;
mod cognition;
mod connection;
mod continuity;
mod doctor;
mod experience;
mod follow;
mod host;
mod level;
mod mcp;
mod memory;
mod nature;
mod peer;
mod persona;
mod pressure;
mod project;
mod search;
mod seed;
mod sensation;
mod setup;
mod storage;
mod tenant;
mod texture;
mod ticket;
mod trail;
mod urge;

pub(crate) mod event;

pub(crate) use actor::*;
pub(crate) use agent::*;
pub(crate) use bookmark::*;
pub(crate) use bridge::*;
pub(crate) use chronicle::*;
pub(crate) use cognition::*;
pub(crate) use connection::*;
pub(crate) use continuity::*;
pub(crate) use doctor::*;
pub(crate) use event::*;
pub(crate) use experience::*;
pub(crate) use follow::*;
pub(crate) use host::*;
pub(crate) use level::*;
pub(crate) use mcp::*;
pub(crate) use memory::*;
pub(crate) use nature::*;
pub(crate) use peer::*;
pub(crate) use persona::*;
pub(crate) use pressure::*;
pub(crate) use project::*;
pub(crate) use search::*;
pub(crate) use seed::*;
pub(crate) use sensation::*;
pub(crate) use setup::*;
pub(crate) use storage::*;
pub(crate) use tenant::*;
pub(crate) use texture::*;
pub(crate) use ticket::*;
pub(crate) use trail::*;
pub(crate) use urge::*;
