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

#[allow(dead_code)]
mod operations {
    use aide::axum::{ApiRouter, routing};
    use axum::{
        Json,
        extract::{Path, Query},
        http::StatusCode,
    };

    use crate::*;

    pub(crate) struct LevelOperations {
        pub(crate) kind: LevelRequestType,
    }

    impl LevelOperations {
        const LABEL: &'static str = "levels";
        const PURPOSE: &'static str = "Define memory retention tiers";

        pub(crate) fn router() -> ApiRouter<ServerState> {
            let mut router = ApiRouter::new();

            for kind in LevelRequestType::all() {
                router = router.merge(Self::new(*kind).route());
            }

            ApiRouter::new().nest(&format!("/{}", Self::LABEL), router)
        }

        pub(crate) fn skills() -> Vec<Skill> {
            let mut skills = vec![];

            for kind in LevelRequestType::all() {
                skills.push(Self::new(*kind).skill())
            }

            skills
        }

        pub(crate) fn new(kind: LevelRequestType) -> Self {
            Self { kind }
        }

        fn content(&self) -> Content {
            match self.kind {
                LevelRequestType::SetLevel => include_str!("features/skills/set.md"),
                LevelRequestType::GetLevel => include_str!("features/skills/show.md"),
                LevelRequestType::ListLevels => include_str!("features/skills/list.md"),
                LevelRequestType::RemoveLevel => include_str!("features/skills/remove.md"),
            }
            .into()
        }

        fn description(&self) -> Description {
            match self.kind {
                LevelRequestType::SetLevel => {
                    "Define or update a named memory retention tier with its priority and eviction policy."
                }
                LevelRequestType::GetLevel => {
                    "Look up the configuration of a specific memory retention level by name."
                }
                LevelRequestType::ListLevels => {
                    "List all memory retention levels defined for the current project."
                }
                LevelRequestType::RemoveLevel => {
                    "Delete a memory retention level, preventing new memories from being classified under it."
                }
            }
            .into()
        }

        fn path(&self) -> &'static str {
            match self.kind {
                LevelRequestType::SetLevel => "/{name}",
                LevelRequestType::GetLevel => "/{name}",
                LevelRequestType::ListLevels => "/",
                LevelRequestType::RemoveLevel => "/{name}",
            }
        }

        fn resource_docs(&self) -> ResourceDocs {
            ResourceDocs::builder()
                .tag(self.tag())
                .nickname(self.kind.to_string())
                .summary(self.summary())
                .description(self.description())
                .content(self.content())
                .build()
        }

        fn route(&self) -> ApiRouter<ServerState> {
            let route = match self.kind {
                LevelRequestType::SetLevel => routing::put_with(set, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .input::<NamePathParam<LevelName>>()
                        .response::<200, Json<LevelSetResponse>>()
                }),
                LevelRequestType::GetLevel => routing::get_with(show, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .input::<NamePathParam<LevelName>>()
                        .response::<200, Json<LevelDetailsResponse>>()
                }),
                LevelRequestType::ListLevels => routing::get_with(list, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<200, Json<LevelsResponse>>()
                }),
                LevelRequestType::RemoveLevel => routing::delete_with(remove, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .input::<NamePathParam<LevelName>>()
                        .response::<200, Json<LevelRemovedResponse>>()
                }),
            };

            ApiRouter::new().api_route(self.path(), route)
        }

        fn skill(&self) -> Skill {
            Skill::builder()
                .name(std::borrow::Cow::Owned(self.kind.to_string()))
                .content(std::borrow::Cow::Owned(self.content().0))
                .build()
        }

        fn summary(&self) -> Description {
            match self.kind {
                LevelRequestType::SetLevel => "Set a level",
                LevelRequestType::GetLevel => "Get a level",
                LevelRequestType::ListLevels => "List levels",
                LevelRequestType::RemoveLevel => "Remove a level",
            }
            .into()
        }

        fn tag(&self) -> Tag {
            Tag::builder()
                .name(Self::LABEL)
                .description(Self::PURPOSE)
                .build()
        }
    }

    async fn set(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<LevelName>,
        Json(body): Json<SetLevel>,
    ) -> Result<(StatusCode, Json<LevelResponse>), LevelError> {
        let SetLevel::V1(mut setting) = body;
        setting.name = name;
        let request = SetLevel::V1(setting);
        Ok((
            StatusCode::OK,
            Json(LevelService::set(&scope, &mailbox, &request).await?),
        ))
    }

    async fn list(
        scope: Scope<AtBookmark>,
        Query(params): Query<ListLevels>,
    ) -> Result<Json<LevelResponse>, LevelError> {
        Ok(Json(LevelService::list(&scope, &params).await?))
    }

    async fn show(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<LevelName>>,
    ) -> Result<Json<LevelResponse>, LevelError> {
        Ok(Json(
            LevelService::get(&scope, &GetLevel::builder_v1().key(key).build().into()).await?,
        ))
    }

    async fn remove(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<LevelName>,
    ) -> Result<Json<LevelResponse>, LevelError> {
        Ok(Json(
            LevelService::remove(
                &scope,
                &mailbox,
                &RemoveLevel::builder_v1().name(name).build().into(),
            )
            .await?,
        ))
    }
}
