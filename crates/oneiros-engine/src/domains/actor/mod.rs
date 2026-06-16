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

    use aide::axum::routing::{self, ApiMethodRouter};
    use axum::Json;

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

        fn http_handler(&self) -> ApiMethodRouter<ServerState> {
            match self.kind {
                ActorRequestType::CreateActor => routing::post_with(CreateActor::handler, |op| {
                    resource_op!(op, self).response::<201, Json<ActorCreatedResponse>>()
                }),
                ActorRequestType::GetActor => routing::get_with(GetActor::handler, |op| {
                    resource_op!(op, self)
                        .input::<IdPathParam<ActorId>>()
                        .response::<200, Json<ActorFoundResponse>>()
                }),
                ActorRequestType::ListActors => routing::get_with(ListActors::handler, |op| {
                    resource_op!(op, self).response::<200, Json<ActorsResponse>>()
                }),
            }
        }
    }
}
