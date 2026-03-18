mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::SensationClient;
pub use features::{SensationProjections, SensationRouter};
pub use features::mcp as sensation_mcp;
pub use model::{Sensation, SensationName};
pub use protocol::{
    SensationError, SensationEvents, SensationRemoved, SensationRequest, SensationResponse,
};
pub use repo::SensationRepo;
pub use service::SensationService;
