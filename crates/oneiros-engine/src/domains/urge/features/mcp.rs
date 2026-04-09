use crate::*;

pub struct UrgeTools;

impl UrgeTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        urge_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        urge_mcp::dispatch(context, tool_name, params).await
    }
}

mod urge_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetUrge>::new(UrgeRequestType::SetUrge, "Define a cognitive drive").def(),
            Tool::<GetUrge>::new(UrgeRequestType::GetUrge, "Look up a cognitive drive").def(),
            Tool::<ListUrges>::new(UrgeRequestType::ListUrges, "See all cognitive drives").def(),
            Tool::<RemoveUrge>::new(UrgeRequestType::RemoveUrge, "Remove a cognitive drive").def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: UrgeRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            UrgeRequestType::SetUrge => {
                UrgeService::set(context, &serde_json::from_str(params)?).await
            }
            UrgeRequestType::GetUrge => {
                UrgeService::get(context, &serde_json::from_str(params)?).await
            }
            UrgeRequestType::ListUrges => {
                UrgeService::list(context, &serde_json::from_str(params)?).await
            }
            UrgeRequestType::RemoveUrge => {
                UrgeService::remove(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
