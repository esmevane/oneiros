mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::NatureClient;
pub use features::{NatureProjections, NatureRouter};
pub use features::mcp as nature_mcp;
pub use model::{Nature, NatureName};
pub use protocol::{NatureError, NatureEvents, NatureRemoved, NatureRequest, NatureResponse};
pub use repo::NatureRepo;
pub use service::NatureService;
