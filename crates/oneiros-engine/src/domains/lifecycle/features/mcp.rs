//! Lifecycle MCP driving adapter — translates tool calls into domain service calls.

use crate::*;

#[derive(serde::Deserialize)]
struct AgentParam {
    agent: String,
}

#[derive(serde::Deserialize)]
struct SenseParams {
    agent: String,
    content: String,
}

pub fn tool_defs() -> &'static [ToolDef] {
    &[
        ToolDef {
            name: "dream",
            description: "Restore an agent's full identity and cognitive context",
        },
        ToolDef {
            name: "introspect",
            description: "Look inward — consolidate what matters",
        },
        ToolDef {
            name: "reflect",
            description: "Pause on something significant",
        },
        ToolDef {
            name: "sense",
            description: "Receive and interpret something from outside",
        },
        ToolDef {
            name: "sleep",
            description: "End a session — capture continuity before resting",
        },
    ]
}

pub fn tool_names() -> &'static [&'static str] {
    &["dream", "introspect", "reflect", "sense", "sleep"]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "dream" => {
            let p: AgentParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = LifecycleService::dream(ctx, &p.agent)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "introspect" => {
            let p: AgentParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = LifecycleService::introspect(ctx, &p.agent)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "reflect" => {
            let p: AgentParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = LifecycleService::reflect(ctx, &p.agent)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "sense" => {
            let p: SenseParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = LifecycleService::sense(ctx, &p.agent, &p.content)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "sleep" => {
            let p: AgentParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = LifecycleService::sleep(ctx, &p.agent)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
