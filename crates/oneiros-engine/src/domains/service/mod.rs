mod features;
mod protocol;
#[expect(
    clippy::module_inception,
    reason = "We have a resource called service, and pattern called service"
)]
mod service;

pub use features::*;
pub use protocol::*;
pub use service::*;
