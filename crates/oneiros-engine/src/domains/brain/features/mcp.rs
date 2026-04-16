use crate::*;

pub struct BrainMcp;

impl BrainMcp {
    pub fn defs(&self) -> Vec<ToolDef> {
        brain_mcp::tool_defs()
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

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateBrain>::new(BrainRequestType::CreateBrain, "Create a new brain").def(),
            Tool::<GetBrain>::new(
                BrainRequestType::GetBrain,
                "Look up a specific brain by name",
            )
            .def(),
            Tool::<ListBrains>::new(BrainRequestType::ListBrains, "List all brains").def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: BrainRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let system = SystemContext::new(context.config.clone());

        let value = match request_type {
            BrainRequestType::CreateBrain => {
                BrainService::create(&system, &serde_json::from_str(params)?).await
            }
            BrainRequestType::GetBrain => {
                BrainService::get(&system, &serde_json::from_str(params)?).await
            }
            BrainRequestType::ListBrains => {
                BrainService::list(&system, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
