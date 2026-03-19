//! Experience MCP driving adapter — translates tool calls into domain service calls.

use crate::*;

#[derive(serde::Deserialize)]
struct IdParam {
    id: String,
}

#[derive(serde::Deserialize)]
struct CreateExperienceParams {
    agent: String,
    sensation: String,
    description: String,
}

#[derive(serde::Deserialize)]
struct ListExperiencesParams {
    agent: Option<String>,
}

#[derive(serde::Deserialize)]
struct UpdateDescriptionParams {
    id: String,
    description: String,
}

#[derive(serde::Deserialize)]
struct UpdateSensationParams {
    id: String,
    sensation: String,
}

pub fn tool_defs() -> &'static [ToolDef] {
    &[
        ToolDef {
            name: "create_experience",
            description: "Mark a meaningful moment",
        },
        ToolDef {
            name: "get_experience",
            description: "Revisit a specific experience",
        },
        ToolDef {
            name: "list_experiences",
            description: "Survey threads of meaning",
        },
        ToolDef {
            name: "update_experience_description",
            description: "Refine an experience's description",
        },
        ToolDef {
            name: "update_experience_sensation",
            description: "Refine an experience's sensation",
        },
    ]
}

pub fn tool_names() -> &'static [&'static str] {
    &[
        "create_experience",
        "get_experience",
        "list_experiences",
        "update_experience_description",
        "update_experience_sensation",
    ]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "create_experience" => {
            let p: CreateExperienceParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = ExperienceService::create(ctx, p.agent, p.sensation, p.description)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_experience" => {
            let p: IdParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                ExperienceService::get(ctx, &p.id).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_experiences" => {
            let p: ListExperiencesParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = ExperienceService::list(ctx, p.agent.as_deref())
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "update_experience_description" => {
            let p: UpdateDescriptionParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = ExperienceService::update_description(ctx, &p.id, p.description)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "update_experience_sensation" => {
            let p: UpdateSensationParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = ExperienceService::update_sensation(ctx, &p.id, p.sensation)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
