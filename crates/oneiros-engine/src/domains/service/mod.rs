mod features;
mod protocol;
#[expect(
    clippy::module_inception,
    reason = "We have a resource called service, and pattern called service"
)]
mod service;
mod view;

pub(crate) use features::*;
pub(crate) use protocol::*;
pub(crate) use service::*;
pub(crate) use view::*;
