mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::SearchClient;
pub use features::mcp as search_mcp;
pub use features::{SearchCli, SearchCommands, SearchProjections, SearchRouter};
pub use model::SearchResult;
pub use protocol::{SearchError, SearchRequest, SearchResponse, SearchResults};
pub use repo::SearchRepo;
pub use service::SearchService;
