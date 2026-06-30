mod features;
mod model;
mod protocol;
mod repo;
mod service;
mod store;
mod view;

pub(crate) use features::*;
pub(crate) use model::*;
pub(crate) use operations::*;
pub(crate) use protocol::*;
pub(crate) use repo::*;
pub(crate) use service::*;
pub(crate) use store::*;
pub(crate) use view::*;

mod operations {
    use crate::*;

    /// Single point of entry for the actor domain.
    /// Implements ResourceRoot — router() and skills() are auto-impl'd
    /// by iterating ActorRequestType and reading from meta()/route_def().
    pub(crate) struct ActorOperations;

    impl ResourceRoot for ActorOperations {
        type Kind = ActorRequestType;
        const LABEL: &'static str = "actors";
        const PURPOSE: &'static str = "Manage actors within a tenant";
    }
}
