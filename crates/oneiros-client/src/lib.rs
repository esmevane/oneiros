mod client;
mod error;
mod requests;
mod responses;
mod socket;

pub(crate) use socket::SocketClient;

pub use client::Client;
pub use error::{ConnectionError, Error, RequestError, ResponseError};
pub use requests::CreateBrainRequest;
pub use responses::BrainInfo;
