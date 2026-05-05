//! Bus — the streaming substrate for event dispatch.
//!
//! The bus is the single, server-owned channel through which services
//! emit events. A service constructs a tier-typed `Message<T>` carrying
//! its `Scope<T>` and the `Event` to dispatch (a `New(...)` for fresh
//! emissions, a `Stored(...)` for post-append notifications), wraps it
//! as a `RoutedMessage`, and `tell`s the `Mailbox`. The host actor
//! receives every message; it handles host-tier work itself and
//! forwards project/bookmark messages to lazily-spawned children.
//!
//! Dispatch is fire-and-forget: services do not await the actor's
//! work. Reads come back through the eventually-consistent fetch
//! primitive, never through phantom state synthesised at the call site.

mod bookmark_actor;
mod host_actor;
mod inbound_actor;
mod mailbox;
mod project_actor;

pub use bookmark_actor::*;
pub use host_actor::*;
pub use inbound_actor::*;
pub use mailbox::*;
pub use project_actor::*;
