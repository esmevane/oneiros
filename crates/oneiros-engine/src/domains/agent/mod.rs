mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::AgentClient;
pub use features::mcp as agent_mcp;
pub use features::{AgentCli, AgentCommands, AgentProjections, AgentRouter};
pub use model::{Agent, AgentId, AgentName};
pub use protocol::{AgentError, AgentEvents, AgentRemoved, AgentRequest, AgentResponse};
pub use repo::AgentRepo;
pub use service::AgentService;
