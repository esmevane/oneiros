use crate::*;

pub struct BookmarkTools;

impl BookmarkTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        bookmark_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        state: &ServerState,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        bookmark_mcp::dispatch(context, state, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourceDef::new(
            "oneiros-mcp://bookmarks",
            "bookmarks",
            "All bookmarks",
        )]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn read_resource_with_state(
        &self,
        state: &ServerState,
        config: &Config,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        match path {
            "bookmarks" => Some(bookmark_mcp::read_bookmarks(state, config).await),
            _ => None,
        }
    }
}

mod bookmark_mcp {
    use crate::*;

    pub async fn read_bookmarks(state: &ServerState, config: &Config) -> Result<String, ToolError> {
        let response = BookmarkService::list(
            state,
            &config.brain,
            &ListBookmarks {
                filters: SearchFilters::default(),
            },
        )
        .await
        .map_err(Error::from)?;

        let mut md = String::from("# Bookmarks\n\n");
        match response {
            BookmarkResponse::Bookmarks(listed) => {
                for bookmark in &listed.items {
                    md.push_str(&format!("- {}\n", bookmark.name));
                }
            }
            _ => md.push_str("No bookmarks.\n"),
        }
        Ok(md)
    }

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<ListBookmarks>::def(
                BookmarkRequestType::ListBookmarks,
                "List all bookmarks for the current brain",
            ),
            Tool::<CreateBookmark>::def(
                BookmarkRequestType::CreateBookmark,
                "Create a new bookmark — fork the current timeline",
            ),
            Tool::<SwitchBookmark>::def(
                BookmarkRequestType::SwitchBookmark,
                "Switch to a different bookmark",
            ),
            Tool::<MergeBookmark>::def(
                BookmarkRequestType::MergeBookmark,
                "Merge a bookmark into the active bookmark",
            ),
            Tool::<ShareBookmark>::def(
                BookmarkRequestType::ShareBookmark,
                "Mint a distribution ticket for a bookmark and return a shareable oneiros:// URI",
            ),
            Tool::<FollowBookmark>::def(
                BookmarkRequestType::FollowBookmark,
                "Follow a bookmark via a URI",
            ),
            Tool::<CollectBookmark>::def(
                BookmarkRequestType::CollectBookmark,
                "Collect events from a followed bookmark's source",
            ),
            Tool::<UnfollowBookmark>::def(
                BookmarkRequestType::UnfollowBookmark,
                "Remove a follow from a bookmark",
            ),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        state: &ServerState,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        let request_type: BookmarkRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let brain = context.brain_name();

        match request_type {
            BookmarkRequestType::ListBookmarks => {
                let request: ListBookmarks = serde_json::from_str(params).unwrap_or_default();
                let resp = BookmarkService::list(state, brain, &request)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BookmarkResponse::Bookmarks(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for bookmark in &listed.items {
                            body.push_str(&format!("- {}\n", bookmark.name));
                        }
                        Ok(McpResponse::new(body))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            BookmarkRequestType::CreateBookmark => {
                let resp = BookmarkService::create(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BookmarkResponse::Created(created) => Ok(McpResponse::new(format!(
                        "Bookmark created: {}",
                        created.name
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            BookmarkRequestType::SwitchBookmark => {
                let resp = BookmarkService::switch(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BookmarkResponse::Switched(switched) => Ok(McpResponse::new(format!(
                        "Switched to bookmark: {}",
                        switched.name
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            BookmarkRequestType::MergeBookmark => {
                let resp = BookmarkService::merge(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BookmarkResponse::Merged(merged) => Ok(McpResponse::new(format!(
                        "Merged **{}** into **{}**",
                        merged.source, merged.target
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            BookmarkRequestType::ShareBookmark => {
                let resp = BookmarkService::share(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BookmarkResponse::Shared(result) => Ok(McpResponse::new(format!(
                        "Bookmark shared.\n\n**URI:** {}\n**ticket:** {}",
                        result.uri, result.ticket.id
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            BookmarkRequestType::FollowBookmark => {
                let resp = BookmarkService::follow(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BookmarkResponse::Followed(follow) => Ok(McpResponse::new(format!(
                        "Now following bookmark **{}** (follow id: {})",
                        follow.bookmark, follow.id
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            BookmarkRequestType::CollectBookmark => {
                let resp = BookmarkService::collect(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BookmarkResponse::Collected(result) => Ok(McpResponse::new(format!(
                        "Collected {} event(s) from follow {}.",
                        result.events_received, result.follow_id
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            BookmarkRequestType::UnfollowBookmark => {
                let resp = BookmarkService::unfollow(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BookmarkResponse::Unfollowed(unfollowed) => Ok(McpResponse::new(format!(
                        "Unfollowed bookmark **{}** (follow id: {})",
                        unfollowed.bookmark, unfollowed.follow_id
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
        }
    }
}
