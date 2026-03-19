mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::StorageClient;
pub use features::mcp as storage_mcp;
pub use features::{StorageProjections, StorageRouter};
pub use model::{StorageContent, StorageEntry, StorageId, StorageKey, StorageName};
pub use protocol::{BlobRemoved, StorageError, StorageEvents, StorageRequest, StorageResponse};
pub use repo::StorageRepo;
pub use service::StorageService;
