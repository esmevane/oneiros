mod features;
mod protocol;
#[expect(
    clippy::module_inception,
    reason = "We have a resource called service, and pattern called service"
)]
mod service;
mod view;

pub use features::*;
pub use protocol::*;
pub use service::*;
pub use view::*;
