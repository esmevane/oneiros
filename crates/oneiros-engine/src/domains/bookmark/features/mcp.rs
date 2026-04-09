use crate::*;

pub struct BookmarkTools;

impl BookmarkTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        bookmark_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        bookmark_mcp::tool_names()
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

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "list_bookmarks",
                description: "List all bookmarks for the current brain",
                input_schema: schema_for::<ListBookmarks>,
            },
            ToolDef {
                name: "create_bookmark",
                description: "Create a new bookmark — fork the current timeline",
                input_schema: schema_for::<CreateBookmark>,
            },
            ToolDef {
                name: "switch_bookmark",
                description: "Switch to a different bookmark",
                input_schema: schema_for::<SwitchBookmark>,
            },
            ToolDef {
                name: "merge_bookmark",
                description: "Merge a bookmark into the active bookmark",
                input_schema: schema_for::<MergeBookmark>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &[
            "list_bookmarks",
            "create_bookmark",
            "switch_bookmark",
            "merge_bookmark",
        ]
    }

    pub async fn dispatch(
        state: &ServerState,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let system = state.system_context();
        let brain = &state.config().brain;

        let value = match tool_name {
            "list_bookmarks" => {
                let request: ListBookmarks = serde_json::from_str(params).unwrap_or_default();
                BookmarkService::list(&system, brain, &request)
                    .await
                    .map_err(Error::from)?
            }
            "create_bookmark" => BookmarkService::create(
                &system,
                state.canons(),
                brain,
                &serde_json::from_str(params)?,
            )
            .await
            .map_err(Error::from)?,
            "switch_bookmark" => BookmarkService::switch(
                &system,
                state.canons(),
                state.config(),
                brain,
                &serde_json::from_str(params)?,
            )
            .await
            .map_err(Error::from)?,
            "merge_bookmark" => BookmarkService::merge(
                &system,
                state.canons(),
                state.config(),
                brain,
                &serde_json::from_str(params)?,
            )
            .await
            .map_err(Error::from)?,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };

        Ok(serde_json::to_value(value)?)
    }
}
