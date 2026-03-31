use crate::*;

pub struct BrainTools;

impl BrainTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        brain_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        brain_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        brain_mcp::dispatch(context, tool_name, params).await
    }
}

mod brain_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_brain",
                description: "Create a new brain",
                input_schema: schema_for::<CreateBrain>,
            },
            ToolDef {
                name: "get_brain",
                description: "Look up a specific brain by name",
                input_schema: schema_for::<GetBrain>,
            },
            ToolDef {
                name: "list_brains",
                description: "List all brains",
                input_schema: schema_for::<serde_json::Value>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["create_brain", "get_brain", "list_brains"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let system = SystemContext::new(context.config.clone());

        let value = match tool_name {
            "create_brain" => BrainService::create(&system, &serde_json::from_str(params)?).await,
            "get_brain" => BrainService::get(&system, &serde_json::from_str(params)?).await,
            "list_brains" => BrainService::list(&system).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
