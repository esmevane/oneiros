use crate::*;

pub struct MemoryTools;

impl MemoryTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        memory_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        memory_mcp::tool_names()
    }

    pub async fn dispatch(
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

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "add_memory",
                description: "Consolidate something you've learned",
                input_schema: schema_for::<AddMemory>,
            },
            ToolDef {
                name: "get_memory",
                description: "Revisit a specific memory",
                input_schema: schema_for::<GetMemory>,
            },
            ToolDef {
                name: "list_memories",
                description: "Review what you know",
                input_schema: schema_for::<ListMemories>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["add_memory", "get_memory", "list_memories"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "add_memory" => MemoryService::add(context, &serde_json::from_str(params)?).await,
            "get_memory" => MemoryService::get(context, &serde_json::from_str(params)?).await,
            "list_memories" => MemoryService::list(context, &serde_json::from_str(params)?).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
