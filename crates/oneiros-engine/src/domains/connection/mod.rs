mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::ConnectionClient;
pub use features::mcp as connection_mcp;
pub use features::{ConnectionProjections, ConnectionRouter};
pub use model::{Connection, ConnectionId};
pub use protocol::{
    ConnectionError, ConnectionEvents, ConnectionRemoved, ConnectionRequest, ConnectionResponse,
};
pub use repo::ConnectionRepo;
pub use service::ConnectionService;
