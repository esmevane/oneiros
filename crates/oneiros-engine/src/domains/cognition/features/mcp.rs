use crate::*;

pub struct CognitionTools;

impl CognitionTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        cognition_mcp::tool_defs()
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

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<AddCognition>::new(CognitionRequestType::AddCognition, "Record a thought").def(),
            Tool::<GetCognition>::new(
                CognitionRequestType::GetCognition,
                "Revisit a specific thought",
            )
            .def(),
            Tool::<ListCognitions>::new(
                CognitionRequestType::ListCognitions,
                "Review a stream of thoughts",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: CognitionRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            CognitionRequestType::AddCognition => {
                CognitionService::add(context, &serde_json::from_str(params)?).await
            }
            CognitionRequestType::GetCognition => {
                CognitionService::get(context, &serde_json::from_str(params)?).await
            }
            CognitionRequestType::ListCognitions => {
                CognitionService::list(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
