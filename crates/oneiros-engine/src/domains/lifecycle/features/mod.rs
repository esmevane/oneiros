mod cli;
mod http;
pub mod mcp;

pub use cli::{LifecycleCli, LifecycleCommands};
pub use http::LifecycleRouter;
