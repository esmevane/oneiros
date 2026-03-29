pub mod persona_mcp {
    //! Persona MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct NameParam {
        name: PersonaName,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_persona",
                description: "Define a category of agent",
                input_schema: schema_for::<Persona>,
            },
            ToolDef {
                name: "get_persona",
                description: "Look up an agent category",
                input_schema: schema_for::<NameParam>,
            },
            ToolDef {
                name: "list_personas",
                description: "See all agent categories",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "remove_persona",
                description: "Remove an agent category",
                input_schema: schema_for::<NameParam>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &[
            "set_persona",
            "get_persona",
            "list_personas",
            "remove_persona",
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "set_persona" => {
                let persona: Persona = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = PersonaService::set(context, persona)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_persona" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = PersonaService::get(context, &p.name)
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_personas" => {
                let response =
                    PersonaService::list(context).map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "remove_persona" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = PersonaService::remove(context, &p.name)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
