pub mod cognition_mcp {
    //! Cognition MCP driving adapter — translates tool calls into domain service calls.

    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct IdParam {
        id: CognitionId,
    }

    #[derive(Deserialize, JsonSchema)]
    struct AddCognitionParams {
        agent: AgentName,
        texture: TextureName,
        content: Content,
    }

    #[derive(Deserialize, JsonSchema)]
    struct ListCognitionsParams {
        agent: Option<AgentName>,
        texture: Option<TextureName>,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "add_cognition",
                description: "Record a thought",
                input_schema: schema_for::<AddCognitionParams>,
            },
            ToolDef {
                name: "get_cognition",
                description: "Revisit a specific thought",
                input_schema: schema_for::<IdParam>,
            },
            ToolDef {
                name: "list_cognitions",
                description: "Review a stream of thoughts",
                input_schema: schema_for::<ListCognitionsParams>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["add_cognition", "get_cognition", "list_cognitions"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "add_cognition" => {
                let params: AddCognitionParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response =
                    CognitionService::add(context, params.agent, params.texture, params.content)
                        .await
                        .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_cognition" => {
                let params: IdParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = CognitionService::get(context, &params.id)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_cognitions" => {
                let params: ListCognitionsParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = CognitionService::list(context, params.agent, params.texture)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
