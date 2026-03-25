pub mod experience_mcp {
    //! Experience MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct IdParam {
        id: String,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct CreateExperienceParams {
        agent: String,
        sensation: String,
        description: String,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct ListExperiencesParams {
        agent: Option<String>,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct UpdateDescriptionParams {
        id: String,
        description: String,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct UpdateSensationParams {
        id: String,
        sensation: String,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_experience",
                description: "Mark a meaningful moment",
                input_schema: schema_for::<CreateExperienceParams>,
            },
            ToolDef {
                name: "get_experience",
                description: "Revisit a specific experience",
                input_schema: schema_for::<IdParam>,
            },
            ToolDef {
                name: "list_experiences",
                description: "Survey threads of meaning",
                input_schema: schema_for::<ListExperiencesParams>,
            },
            ToolDef {
                name: "update_experience_description",
                description: "Refine an experience's description",
                input_schema: schema_for::<UpdateDescriptionParams>,
            },
            ToolDef {
                name: "update_experience_sensation",
                description: "Refine an experience's sensation",
                input_schema: schema_for::<UpdateSensationParams>,
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

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "create_experience" => {
                let p: CreateExperienceParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ExperienceService::create(
                    context,
                    &AgentName::new(&p.agent),
                    SensationName::new(&p.sensation),
                    Description::new(&p.description),
                )
                .await
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_experience" => {
                let p: IdParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let id: ExperienceId =
                    p.id.parse()
                        .map_err(|e: IdParseError| ToolError::Parameter(e.to_string()))?;
                let response = ExperienceService::get(context, &id)
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_experiences" => {
                let p: ListExperiencesParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ExperienceService::list(
                    context,
                    p.agent.as_deref().map(AgentName::new).as_ref(),
                )
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "update_experience_description" => {
                let p: UpdateDescriptionParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let id: ExperienceId =
                    p.id.parse()
                        .map_err(|e: IdParseError| ToolError::Parameter(e.to_string()))?;
                let response = ExperienceService::update_description(
                    context,
                    &id,
                    Description::new(&p.description),
                )
                .await
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "update_experience_sensation" => {
                let p: UpdateSensationParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let id: ExperienceId =
                    p.id.parse()
                        .map_err(|e: IdParseError| ToolError::Parameter(e.to_string()))?;
                let response = ExperienceService::update_sensation(
                    context,
                    &id,
                    SensationName::new(&p.sensation),
                )
                .await
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
