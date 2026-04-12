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
    ) -> Result<serde_json::Value, ToolError> {
        bookmark_mcp::dispatch(context, state, tool_name, params).await
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
            Tool::<FollowBookmark>::new(
                BookmarkRequestType::FollowBookmark,
                "Follow a bookmark via a URI",
            )
            .def(),
            Tool::<CollectBookmark>::new(
                BookmarkRequestType::CollectBookmark,
                "Collect events from a followed bookmark's source",
            )
            .def(),
            Tool::<UnfollowBookmark>::new(
                BookmarkRequestType::UnfollowBookmark,
                "Remove a follow from a bookmark",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        state: &ServerState,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: BookmarkRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let brain = context.brain_name();

        let value = match request_type {
            BookmarkRequestType::ListBookmarks => {
                let request: ListBookmarks = serde_json::from_str(params).unwrap_or_default();
                BookmarkService::list(state, brain, &request)
                    .await
                    .map_err(Error::from)?
            }
            BookmarkRequestType::CreateBookmark => {
                BookmarkService::create(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?
            }
            BookmarkRequestType::SwitchBookmark => {
                BookmarkService::switch(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?
            }
            BookmarkRequestType::MergeBookmark => {
                BookmarkService::merge(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?
            }
            BookmarkRequestType::ShareBookmark => {
                BookmarkService::share(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?
            }
            BookmarkRequestType::FollowBookmark => {
                BookmarkService::follow(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?
            }
            BookmarkRequestType::CollectBookmark => {
                BookmarkService::collect(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?
            }
            BookmarkRequestType::UnfollowBookmark => {
                BookmarkService::unfollow(state, brain, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?
            }
        };

        Ok(serde_json::to_value(value)?)
    }
}
