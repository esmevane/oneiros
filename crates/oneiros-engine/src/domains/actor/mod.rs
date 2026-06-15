mod docs;
mod features;
mod model;
mod protocol;
mod repo;
mod service;
mod store;
mod view;

pub(crate) use docs::*;
pub(crate) use features::*;
pub(crate) use model::*;
pub(crate) use operations::*;
pub(crate) use protocol::*;
pub(crate) use repo::*;
pub(crate) use service::*;
pub(crate) use store::*;
pub(crate) use view::*;

mod operations {
    use aide::axum::{ApiRouter, routing};
    use axum::{
        Json,
        extract::{Path, Query},
    };
    use reqwest::StatusCode;

    use crate::*;

    pub(crate) struct ActorOperations {
        pub(crate) kind: ActorRequestType,
    }

    impl ActorOperations {
        const LABEL: &'static str = "actors";
        const PURPOSE: &'static str = "Manage actors within a tenant";

        pub(crate) fn router() -> ApiRouter<ServerState> {
            let mut router = ApiRouter::new();

            for kind in ActorRequestType::all() {
                router = router.merge(Self::new(*kind).route());
            }

            ApiRouter::new().nest(&format!("/{}", Self::LABEL), router)
        }

        pub(crate) fn skills() -> Vec<Skill> {
            let mut skills = vec![];

            for kind in ActorRequestType::all() {
                skills.push(Self::new(*kind).skill())
            }

            skills
        }

        pub(crate) fn new(kind: ActorRequestType) -> Self {
            Self { kind }
        }

        fn content(&self) -> Content {
            match self.kind {
                ActorRequestType::CreateActor => include_str!("features/skills/create.md"),
                ActorRequestType::GetActor => include_str!("features/skills/get.md"),
                ActorRequestType::ListActors => include_str!("features/skills/list.md"),
            }
            .into()
        }

        fn label(&self) -> Label {
            self.kind.to_string().into()
        }

        fn description(&self) -> Description {
            match self.kind {
                ActorRequestType::CreateActor => "Register a new actor under the current tenant.",
                ActorRequestType::GetActor => "Look up a specific actor by ID.",
                ActorRequestType::ListActors => "List all actors for a tenant.",
            }
            .into()
        }

        fn skill(&self) -> Skill {
            Skill::builder()
                .name(self.kind.to_string())
                .content(self.content().to_string())
                .build()
        }

        fn summary(&self) -> Description {
            match self.kind {
                ActorRequestType::CreateActor => "Create an actor",
                ActorRequestType::GetActor => "Get an actor",
                ActorRequestType::ListActors => "List actors",
            }
            .into()
        }

        fn path(&self) -> &'static str {
            match self.kind {
                ActorRequestType::CreateActor => "/",
                ActorRequestType::GetActor => "/{id}",
                ActorRequestType::ListActors => "/",
            }
        }

        fn resource_docs(&self) -> ResourceDocs {
            ResourceDocs::builder()
                .tag(self.tag())
                .nickname(self.label())
                .summary(self.summary())
                .description(self.description())
                .content(self.content())
                .build()
        }

        fn route(&self) -> ApiRouter<ServerState> {
            let route = match self.kind {
                ActorRequestType::CreateActor => routing::post_with(create, |op| {
                    resource_op!(op, self).response::<201, Json<ActorCreatedResponse>>()
                }),
                ActorRequestType::GetActor => routing::get_with(list, |op| {
                    resource_op!(op, self).response::<200, Json<ActorsResponse>>()
                }),
                ActorRequestType::ListActors => routing::get_with(show, |op| {
                    resource_op!(op, self)
                        .input::<IdPathParam<ActorId>>()
                        .response::<200, Json<ActorFoundResponse>>()
                }),
            };

            ApiRouter::new().api_route(self.path(), route)
        }

        fn tag(&self) -> Tag {
            Tag::builder()
                .name(Self::LABEL)
                .description(Self::PURPOSE)
                .build()
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
