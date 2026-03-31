use crate::*;

pub struct PersonaTools;

impl PersonaTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        persona_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        persona_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        persona_mcp::dispatch(context, tool_name, params).await
    }
}

mod persona_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_persona",
                description: "Define a category of agent",
                input_schema: schema_for::<SetPersona>,
            },
            ToolDef {
                name: "get_persona",
                description: "Look up an agent category",
                input_schema: schema_for::<GetPersona>,
            },
            ToolDef {
                name: "list_personas",
                description: "See all agent categories",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "remove_persona",
                description: "Remove an agent category",
                input_schema: schema_for::<RemovePersona>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
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
            "set_persona" => PersonaService::set(context, &serde_json::from_str(params)?).await,
            "get_persona" => PersonaService::get(context, &serde_json::from_str(params)?).await,
            "list_personas" => PersonaService::list(context).await,
            "remove_persona" => {
                PersonaService::remove(context, &serde_json::from_str(params)?).await
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
