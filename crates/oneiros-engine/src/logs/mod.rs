//! Logs — proto-Currents publish substrate.
//!
//! `HostLog` and `ProjectLog` are the typed event-log surfaces for
//! their respective tiers. Today they hold:
//! - emit (publish + apply projections inline)
//! - chronicle (project-tier only)
//!
//! These are the seeds of Currents. When subscriber bedrock lands,
//! their bodies decompose: emit becomes publish-to-stream, projection
//! apply / chronicle write each become Subscribers.
//! The Log types themselves survive — only their internals change.

mod host;
mod project;

pub use host::HostLog;
pub use project::ProjectLog;
