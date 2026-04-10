use crate::*;

pub struct BookmarkTools;

impl BookmarkTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        bookmark_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        bookmark_mcp::dispatch(state, tool_name, params).await
    }
}

mod bookmark_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<ListBookmarks>::new(
                BookmarkRequestType::ListBookmarks,
                "List all bookmarks for the current brain",
            )
            .def(),
            Tool::<CreateBookmark>::new(
                BookmarkRequestType::CreateBookmark,
                "Create a new bookmark — fork the current timeline",
            )
            .def(),
            Tool::<SwitchBookmark>::new(
                BookmarkRequestType::SwitchBookmark,
                "Switch to a different bookmark",
            )
            .def(),
            Tool::<MergeBookmark>::new(
                BookmarkRequestType::MergeBookmark,
                "Merge a bookmark into the active bookmark",
            )
            .def(),
            Tool::<ShareBookmark>::new(
                BookmarkRequestType::ShareBookmark,
                "Mint a distribution ticket for a bookmark and return a shareable oneiros:// URI",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        state: &ServerState,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: BookmarkRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let system = state.system_context();
        let brain = &state.config().brain;

        let value = match request_type {
            BookmarkRequestType::ListBookmarks => {
                let request: ListBookmarks = serde_json::from_str(params).unwrap_or_default();
                BookmarkService::list(&system, brain, &request)
                    .await
                    .map_err(Error::from)?
            }
            BookmarkRequestType::CreateBookmark => BookmarkService::create(
                &system,
                state.canons(),
                brain,
                &serde_json::from_str(params)?,
            )
            .await
            .map_err(Error::from)?,
            BookmarkRequestType::SwitchBookmark => BookmarkService::switch(
                &system,
                state.canons(),
                state.config(),
                brain,
                &serde_json::from_str(params)?,
            )
            .await
            .map_err(Error::from)?,
            BookmarkRequestType::MergeBookmark => BookmarkService::merge(
                &system,
                state.canons(),
                state.config(),
                brain,
                &serde_json::from_str(params)?,
            )
            .await
            .map_err(Error::from)?,
            BookmarkRequestType::ShareBookmark => {
                let identity = state
                    .host_identity()
                    .ok_or_else(|| ToolError::Domain("server not bound to a bridge".into()))?;
                BookmarkService::share(&system, identity, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?
            }
            // Follow/Collect/Unfollow land in subsequent Act 3 slices.
            BookmarkRequestType::FollowBookmark
            | BookmarkRequestType::CollectBookmark
            | BookmarkRequestType::UnfollowBookmark => {
                return Err(ToolError::UnknownTool(format!(
                    "{tool_name} is not yet wired"
                )));
            }
        };

        Ok(serde_json::to_value(value)?)
    }
}
