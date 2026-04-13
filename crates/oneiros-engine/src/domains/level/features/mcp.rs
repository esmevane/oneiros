use crate::*;

pub(crate) struct LevelTools;

impl LevelTools {
    pub(crate) fn defs(&self) -> Vec<ToolDef> {
        level_mcp::tool_defs()
    }

    pub(crate) async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        level_mcp::dispatch(context, tool_name, params).await
    }
}

mod level_mcp {
    use crate::*;

    pub(crate) fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetLevel>::new(
                LevelRequestType::SetLevel,
                "Define how long a kind of memory should be kept",
            )
            .def(),
            Tool::<GetLevel>::new(
                LevelRequestType::GetLevel,
                "Look up a memory retention tier",
            )
            .def(),
            Tool::<ListLevels>::new(
                LevelRequestType::ListLevels,
                "See all memory retention tiers",
            )
            .def(),
            Tool::<RemoveLevel>::new(
                LevelRequestType::RemoveLevel,
                "Remove a memory retention tier",
            )
            .def(),
        ]
    }

    pub(crate) async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: LevelRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            LevelRequestType::SetLevel => {
                LevelService::set(context, &serde_json::from_str(params)?).await
            }
            LevelRequestType::GetLevel => {
                LevelService::get(context, &serde_json::from_str(params)?).await
            }
            LevelRequestType::ListLevels => {
                LevelService::list(context, &serde_json::from_str(params)?).await
            }
            LevelRequestType::RemoveLevel => {
                LevelService::remove(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
