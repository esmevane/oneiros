mod cli;
mod http;
pub mod mcp;
mod projections;

pub use cli::{SensationCli, SensationCommands};
pub use http::SensationRouter;
pub use projections::SensationProjections;
