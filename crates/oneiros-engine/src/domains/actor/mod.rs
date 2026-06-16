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
    use axum::{
        Json,
        extract::{Path, Query},
    };
    use reqwest::StatusCode;

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
                ActorRequestType::CreateActor => routing::post_with(create, |op| {
                    resource_op!(op, self).response::<201, Json<ActorCreatedResponse>>()
                }),
                ActorRequestType::GetActor => routing::get_with(show, |op| {
                    resource_op!(op, self)
                        .input::<IdPathParam<ActorId>>()
                        .response::<200, Json<ActorFoundResponse>>()
                }),
                ActorRequestType::ListActors => routing::get_with(list, |op| {
                    resource_op!(op, self).response::<200, Json<ActorsResponse>>()
                }),
            }
        }
    }

    async fn create(
        scope: Scope<AtHost>,
        mailbox: Mailbox,
        Json(body): Json<CreateActor>,
    ) -> Result<(StatusCode, Json<ActorResponse>), ActorError> {
        let response = ActorService::create(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }

    async fn list(
        scope: Scope<AtHost>,
        Query(params): Query<ListActors>,
    ) -> Result<Json<ActorResponse>, ActorError> {
        Ok(Json(ActorService::list(&scope, &params).await?))
    }

    async fn show(
        scope: Scope<AtHost>,
        Path(key): Path<ResourceKey<ActorId>>,
    ) -> Result<Json<ActorResponse>, ActorError> {
        Ok(Json(
            ActorService::get(&scope, &GetActor::builder_v1().key(key).build().into()).await?,
        ))
    }
}
