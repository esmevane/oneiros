pub mod cognition_mcp {
    //! Cognition MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct IdParam {
        id: String,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct AddCognitionParams {
        agent: String,
        texture: String,
        content: String,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct ListCognitionsParams {
        agent: Option<String>,
        texture: Option<String>,
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
                let p: AddCognitionParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = CognitionService::add(
                    context,
                    &AgentName::new(&p.agent),
                    TextureName::new(&p.texture),
                    Content::new(p.content),
                )
                .await
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_cognition" => {
                let p: IdParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let id: CognitionId =
                    p.id.parse()
                        .map_err(|e: IdParseError| ToolError::Parameter(e.to_string()))?;
                let response = CognitionService::get(context, &id)
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_cognitions" => {
                let p: ListCognitionsParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = CognitionService::list(
                    context,
                    p.agent.as_deref().map(AgentName::new).as_ref(),
                    p.texture.as_deref().map(TextureName::new).as_ref(),
                )
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
