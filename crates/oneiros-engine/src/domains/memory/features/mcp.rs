use crate::*;

pub(crate) struct MemoryTools;

impl MemoryTools {
    pub(crate) fn defs(&self) -> Vec<ToolDef> {
        memory_mcp::tool_defs()
    }

    pub(crate) async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        memory_mcp::dispatch(context, tool_name, params).await
    }
}

mod memory_mcp {
    use crate::*;

    pub(crate) fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<AddMemory>::new(
                MemoryRequestType::AddMemory,
                "Consolidate something you've learned",
            )
            .def(),
            Tool::<GetMemory>::new(MemoryRequestType::GetMemory, "Revisit a specific memory").def(),
            Tool::<ListMemories>::new(MemoryRequestType::ListMemories, "Review what you know")
                .def(),
        ]
    }

    pub(crate) async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: MemoryRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            MemoryRequestType::AddMemory => {
                MemoryService::add(context, &serde_json::from_str(params)?).await
            }
            MemoryRequestType::GetMemory => {
                MemoryService::get(context, &serde_json::from_str(params)?).await
            }
            MemoryRequestType::ListMemories => {
                MemoryService::list(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
