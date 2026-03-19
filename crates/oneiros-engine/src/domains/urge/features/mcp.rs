//! Urge MCP driving adapter — translates tool calls into domain service calls.

use crate::*;

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct NameParam {
    name: String,
}

pub fn tool_defs() -> &'static [ToolDef] {
    &[
        ToolDef {
            name: "set_urge",
            description: "Define a cognitive drive",
            input_schema: schema_for::<Urge>,
        },
        ToolDef {
            name: "get_urge",
            description: "Look up a cognitive drive",
            input_schema: schema_for::<NameParam>,
        },
        ToolDef {
            name: "list_urges",
            description: "See all cognitive drives",
            input_schema: schema_for::<serde_json::Value>,
        },
        ToolDef {
            name: "remove_urge",
            description: "Remove a cognitive drive",
            input_schema: schema_for::<NameParam>,
        },
    ]
}

pub fn tool_names() -> &'static [&'static str] {
    &["set_urge", "get_urge", "list_urges", "remove_urge"]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "set_urge" => {
            let urge: Urge =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                UrgeService::set(ctx, urge).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_urge" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                UrgeService::get(ctx, &p.name).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_urges" => {
            let response = UrgeService::list(ctx).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "remove_urge" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                UrgeService::remove(ctx, &p.name).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
