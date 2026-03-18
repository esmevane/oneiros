mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::CognitionClient;
pub use features::mcp as cognition_mcp;
pub use features::{CognitionCli, CognitionCommands, CognitionProjections, CognitionRouter};
pub use model::{Cognition, CognitionId};
pub use protocol::{CognitionError, CognitionEvents, CognitionRequest, CognitionResponse};
pub use repo::CognitionRepo;
pub use service::CognitionService;
