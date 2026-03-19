//! Memory MCP driving adapter — translates tool calls into domain service calls.

use crate::*;

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct IdParam {
    id: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct AddMemoryParams {
    agent: String,
    level: String,
    content: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct ListMemoriesParams {
    agent: Option<String>,
}

pub fn tool_defs() -> &'static [ToolDef] {
    &[
        ToolDef {
            name: "add_memory",
            description: "Consolidate something you've learned",
            input_schema: schema_for::<AddMemoryParams>,
        },
        ToolDef {
            name: "get_memory",
            description: "Revisit a specific memory",
            input_schema: schema_for::<IdParam>,
        },
        ToolDef {
            name: "list_memories",
            description: "Review what you know",
            input_schema: schema_for::<ListMemoriesParams>,
        },
    ]
}

pub fn tool_names() -> &'static [&'static str] {
    &["add_memory", "get_memory", "list_memories"]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "add_memory" => {
            let p: AddMemoryParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = MemoryService::add(ctx, p.agent, p.level, p.content)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_memory" => {
            let p: IdParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                MemoryService::get(ctx, &p.id).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_memories" => {
            let p: ListMemoriesParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = MemoryService::list(ctx, p.agent.as_deref())
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
