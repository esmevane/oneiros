//! Agent MCP driving adapter — translates tool calls into domain service calls.

use crate::*;

#[derive(serde::Deserialize)]
struct NameParam {
    name: String,
}

#[derive(serde::Deserialize)]
struct AgentParams {
    name: String,
    persona: String,
    description: String,
    prompt: String,
}

pub fn tool_defs() -> &'static [ToolDef] {
    &[
        ToolDef {
            name: "create_agent",
            description: "Bring a new agent into the brain",
        },
        ToolDef {
            name: "get_agent",
            description: "Learn about a specific agent",
        },
        ToolDef {
            name: "list_agents",
            description: "See who's here",
        },
        ToolDef {
            name: "update_agent",
            description: "Reshape an agent's identity",
        },
        ToolDef {
            name: "remove_agent",
            description: "Remove an agent from the brain",
        },
    ]
}

pub fn tool_names() -> &'static [&'static str] {
    &[
        "create_agent",
        "get_agent",
        "list_agents",
        "update_agent",
        "remove_agent",
    ]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "create_agent" => {
            let p: AgentParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = AgentService::create(ctx, p.name, p.persona, p.description, p.prompt)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_agent" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                AgentService::get(ctx, &p.name).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_agents" => {
            let response = AgentService::list(ctx).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "update_agent" => {
            let p: AgentParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = AgentService::update(ctx, p.name, p.persona, p.description, p.prompt)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "remove_agent" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                AgentService::remove(ctx, &p.name).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
