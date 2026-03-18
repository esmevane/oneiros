mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::MemoryClient;
pub use features::mcp as memory_mcp;
pub use features::{MemoryCli, MemoryCommands, MemoryProjections, MemoryRouter};
pub use model::{Memory, MemoryId};
pub use protocol::{MemoryError, MemoryEvents, MemoryRequest, MemoryResponse};
pub use repo::MemoryRepo;
pub use service::MemoryService;
