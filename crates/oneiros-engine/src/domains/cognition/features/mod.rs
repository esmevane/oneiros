mod cli;
pub mod http;
pub mod mcp;
mod projections;

pub use cli::{CognitionCli, CognitionCommands};
pub use http::CognitionRouter;
pub use projections::CognitionProjections;
