mod cli;
mod http;
pub mod mcp;
mod projections;

pub use cli::{SearchCli, SearchCommands};
pub use http::SearchRouter;
pub use projections::SearchProjections;
