mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::UrgeClient;
pub use features::mcp as urge_mcp;
pub use features::{UrgeCli, UrgeCommands, UrgeProjections, UrgeRouter};
pub use model::{Urge, UrgeName};
pub use protocol::{UrgeError, UrgeEvents, UrgeRemoved, UrgeRequest, UrgeResponse};
pub use repo::UrgeRepo;
pub use service::UrgeService;
