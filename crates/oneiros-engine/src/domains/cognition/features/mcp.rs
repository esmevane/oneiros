use crate::*;

pub struct CognitionTools;

impl CognitionTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        cognition_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        cognition_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        cognition_mcp::dispatch(context, tool_name, params).await
    }
}

mod cognition_mcp {
    //! Cognition MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "add_cognition",
                description: "Record a thought",
                input_schema: schema_for::<AddCognition>,
            },
            ToolDef {
                name: "get_cognition",
                description: "Revisit a specific thought",
                input_schema: schema_for::<GetCognition>,
            },
            ToolDef {
                name: "list_cognitions",
                description: "Review a stream of thoughts",
                input_schema: schema_for::<ListCognitions>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["add_cognition", "get_cognition", "list_cognitions"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "add_cognition" => CognitionService::add(context, &serde_json::from_str(params)?).await,
            "get_cognition" => CognitionService::get(context, &serde_json::from_str(params)?).await,
            "list_cognitions" => {
                CognitionService::list(context, &serde_json::from_str(params)?).await
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
