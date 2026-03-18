mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::ExperienceClient;
pub use features::mcp as experience_mcp;
pub use features::{ExperienceCli, ExperienceCommands, ExperienceProjections, ExperienceRouter};
pub use model::{Experience, ExperienceId};
pub use protocol::{
    ExperienceDescriptionUpdate, ExperienceError, ExperienceEvents, ExperienceRequest,
    ExperienceResponse, ExperienceSensationUpdate,
};
pub use repo::ExperienceRepo;
pub use service::ExperienceService;
