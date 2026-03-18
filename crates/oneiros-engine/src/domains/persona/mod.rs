mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::PersonaClient;
pub use features::mcp as persona_mcp;
pub use features::{PersonaCli, PersonaCommands, PersonaProjections, PersonaRouter};
pub use model::{Persona, PersonaName};
pub use protocol::{PersonaError, PersonaEvents, PersonaRemoved, PersonaRequest, PersonaResponse};
pub use repo::PersonaRepo;
pub use service::PersonaService;
