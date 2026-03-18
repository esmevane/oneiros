mod cli;
mod http;
pub mod mcp;
mod projections;

pub use cli::{PersonaCli, PersonaCommands};
pub use http::PersonaRouter;
pub use projections::PersonaProjections;
