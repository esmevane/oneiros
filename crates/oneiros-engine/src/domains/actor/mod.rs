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

    pub(crate) struct ActorOperations {
        pub(crate) kind: ActorRequestType,
    }

    impl DomainDef for ActorOperations {
        type Kind = ActorRequestType;
        type Request = ActorRequest;

        const LABEL: &'static str = "actors";
        const PURPOSE: &'static str = "Manage actors within a tenant";

        fn resource_definition(kind: Self::Kind) -> Self {
            Self { kind }
        }

        fn resource(&self) -> Self::Kind {
            self.kind
        }
    }
}
