mod client;
mod error;
mod socket;

pub(crate) use socket::SocketClient;

pub use client::Client;
pub use error::{ConnectionError, Error, RequestError, ResponseError};
pub use oneiros_protocol::*;
