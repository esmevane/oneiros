mod client;
mod features;
mod model;
mod protocol;
mod repo;
mod service;

pub use client::TicketClient;
pub use features::{TicketProjections, TicketRouter};
pub use model::{Ticket, TicketId};
pub use protocol::{TicketError, TicketEvents, TicketRequest, TicketResponse};
pub use repo::TicketRepo;
pub use service::TicketService;
