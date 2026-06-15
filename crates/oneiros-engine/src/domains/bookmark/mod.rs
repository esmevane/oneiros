mod actors;
mod features;
mod model;
mod protocol;
mod repo;
mod service;
mod store;
mod view;

pub(crate) use actors::*;
pub(crate) use features::*;
pub(crate) use model::*;
pub(crate) use operations::*;
pub(crate) use protocol::*;
pub(crate) use repo::*;
pub(crate) use service::*;
pub(crate) use store::*;
pub(crate) use view::*;

mod operations {
    use std::collections::BTreeMap;

    use aide::axum::{ApiRouter, routing};
    use axum::{
        Json,
        extract::{Query, State},
        http::StatusCode,
    };

    use crate::*;

    pub(crate) struct BookmarkOperations {
        pub(crate) kind: BookmarkRequestType,
    }

    impl BookmarkOperations {
        const LABEL: &'static str = "bookmarks";
        const PURPOSE: &'static str = "Manage timeline bookmarks";

        pub(crate) fn router() -> ApiRouter<ServerState> {
            let mut by_path: BTreeMap<&'static str, Vec<BookmarkRequestType>> = BTreeMap::new();
            for kind in BookmarkRequestType::all() {
                let ops = Self::new(*kind);
                by_path.entry(ops.path()).or_default().push(*kind);
            }

            let mut inner = ApiRouter::<ServerState>::new();
            for (path, kinds) in by_path {
                let methods = kinds
                    .into_iter()
                    .map(|k| Self::new(k).http_handler())
                    .reduce(|a, b| a.merge(b))
                    .unwrap();
                inner = inner.api_route(path, methods);
            }

            ApiRouter::new().nest(&format!("/{}", Self::LABEL), inner)
        }

        pub(crate) fn skills() -> Vec<Skill> {
            let mut skills = vec![];

            for kind in BookmarkRequestType::all() {
                skills.push(Self::new(*kind).skill())
            }

            skills
        }

        pub(crate) fn new(kind: BookmarkRequestType) -> Self {
            Self { kind }
        }

        fn content(&self) -> Content {
            match self.kind {
                BookmarkRequestType::CreateBookmark => include_str!("features/skills/create.md"),
                BookmarkRequestType::SwitchBookmark => include_str!("features/skills/switch.md"),
                BookmarkRequestType::MergeBookmark => include_str!("features/skills/merge.md"),
                BookmarkRequestType::ListBookmarks => include_str!("features/skills/list.md"),
                BookmarkRequestType::ShareBookmark => include_str!("features/skills/share.md"),
                BookmarkRequestType::FollowBookmark => include_str!("features/skills/follow.md"),
                BookmarkRequestType::CollectBookmark => include_str!("features/skills/collect.md"),
                BookmarkRequestType::UnfollowBookmark => {
                    include_str!("features/skills/unfollow.md")
                }
                BookmarkRequestType::SubmitBookmark => include_str!("features/skills/submit.md"),
            }
            .into()
        }

        fn description(&self) -> Description {
            match self.kind {
                BookmarkRequestType::CreateBookmark => {
                    "Create a new bookmark that defines a named view of the event timeline."
                }
                BookmarkRequestType::SwitchBookmark => {
                    "Set the active bookmark, making its timeline view the current working context."
                }
                BookmarkRequestType::MergeBookmark => {
                    "Integrate the events from a bookmark into the current active timeline."
                }
                BookmarkRequestType::ListBookmarks => {
                    "List all bookmarks known to the current project."
                }
                BookmarkRequestType::ShareBookmark => {
                    "Produce a shareable oneiros:// link representing this bookmark"
                }
                BookmarkRequestType::FollowBookmark => {
                    "Create a local bookmark by following a remote oneiros:// link."
                }
                BookmarkRequestType::CollectBookmark => {
                    "Collect events from a followed source or directly from a peer host."
                }
                BookmarkRequestType::UnfollowBookmark => {
                    "Remove a followed bookmark, stopping incremental collection."
                }
                BookmarkRequestType::SubmitBookmark => "Submit a bookmark to a peer host.",
            }
            .into()
        }

        fn path(&self) -> &'static str {
            match self.kind {
                BookmarkRequestType::CreateBookmark => "/",
                BookmarkRequestType::SwitchBookmark => "/switch",
                BookmarkRequestType::MergeBookmark => "/merge",
                BookmarkRequestType::ListBookmarks => "/",
                BookmarkRequestType::ShareBookmark => "/share",
                BookmarkRequestType::FollowBookmark => "/follow",
                BookmarkRequestType::CollectBookmark => "/collect",
                BookmarkRequestType::UnfollowBookmark => "/unfollow",
                BookmarkRequestType::SubmitBookmark => "/submit",
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

        fn skill(&self) -> Skill {
            Skill::builder()
                .name(std::borrow::Cow::Owned(self.kind.to_string()))
                .content(std::borrow::Cow::Owned(self.content().0))
                .build()
        }

        fn summary(&self) -> Description {
            match self.kind {
                BookmarkRequestType::CreateBookmark => "Create a bookmark",
                BookmarkRequestType::SwitchBookmark => "Switch to a bookmark",
                BookmarkRequestType::MergeBookmark => "Merge a bookmark",
                BookmarkRequestType::ListBookmarks => "List bookmarks",
                BookmarkRequestType::ShareBookmark => "Share a bookmark",
                BookmarkRequestType::FollowBookmark => "Follow a bookmark link",
                BookmarkRequestType::CollectBookmark => "Collect events into a bookmark",
                BookmarkRequestType::UnfollowBookmark => "Unfollow a bookmark",
                BookmarkRequestType::SubmitBookmark => "Submit a bookmark to a remote",
            }
            .into()
        }

        fn http_handler(&self) -> aide::axum::routing::ApiMethodRouter<ServerState> {
            match self.kind {
                BookmarkRequestType::CreateBookmark => routing::post_with(create, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<201, Json<BookmarkCreatedResponse>>()
                }),
                BookmarkRequestType::SwitchBookmark => routing::post_with(switch, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<200, Json<BookmarkSwitchedResponse>>()
                }),
                BookmarkRequestType::MergeBookmark => routing::post_with(merge, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<200, Json<BookmarkMergedResponse>>()
                }),
                BookmarkRequestType::ListBookmarks => routing::get_with(list, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<200, Json<Listed<Bookmark>>>()
                }),
                BookmarkRequestType::ShareBookmark => routing::post_with(share, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<200, Json<BookmarkShareResult>>()
                }),
                BookmarkRequestType::FollowBookmark => routing::post_with(follow, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<200, Json<Follow>>()
                }),
                BookmarkRequestType::CollectBookmark => routing::post_with(collect, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<200, Json<BookmarkCollectResult>>()
                }),
                BookmarkRequestType::UnfollowBookmark => routing::post_with(unfollow, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<200, Json<BookmarkUnfollowedResponse>>()
                }),
                BookmarkRequestType::SubmitBookmark => routing::post_with(submit, |op| {
                    resource_op!(op, self)
                        .security_requirement("BearerToken")
                        .response::<200, Json<BookmarkSubmitResult>>()
                }),
            }
        }

        pub(crate) fn tag(&self) -> Tag {
            Tag::builder()
                .name(Self::LABEL)
                .description(Self::PURPOSE)
                .build()
        }
    }

    async fn create(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<CreateBookmark>,
    ) -> Result<(StatusCode, Json<BookmarkResponse>), BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        let response =
            BookmarkService::create(&scope, &state, context.project_name(), &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }

    async fn switch(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<SwitchBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::switch(&scope, &state, context.project_name(), &body).await?,
        ))
    }

    async fn merge(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<MergeBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::merge(&scope, &state, context.project_name(), &body).await?,
        ))
    }

    async fn list(
        context: ProjectLog,
        State(state): State<ServerState>,
        Query(params): Query<ListBookmarks>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::list(&scope, &state, context.project_name(), &params).await?,
        ))
    }

    async fn share(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<ShareBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::share(&scope, &state, context.project_name(), &body).await?,
        ))
    }

    async fn follow(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<FollowBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::follow(&scope, &state, context.project_name(), &body).await?,
        ))
    }

    async fn unfollow(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<UnfollowBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::unfollow(&scope, &state, context.project_name(), &body).await?,
        ))
    }

    async fn collect(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<CollectBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::collect(&scope, &state, context.project_name(), &body).await?,
        ))
    }

    async fn submit(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<SubmitBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::submit(&scope, &state, context.project_name(), &body).await?,
        ))
    }
}
