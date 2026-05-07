use crate::*;

pub(crate) struct BrainMcp;

impl BrainMcp {
    pub(crate) fn defs(&self) -> Vec<ToolDef> {
        brain_mcp::tool_defs()
    }

    pub(crate) async fn dispatch(
        &self,
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        brain_mcp::dispatch(context, mailbox, tool_name, params).await
    }
}

mod brain_mcp {
    use crate::*;

    pub(crate) fn tool_defs() -> Vec<ToolDef> {
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

    pub(crate) async fn dispatch(
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: BrainRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let scope = ComposeScope::new(context.config.clone())
            .host()
            .map_err(Error::from)?;

        let value = match request_type {
            BrainRequestType::CreateBrain => {
                BrainService::create(&scope, mailbox, &serde_json::from_str(params)?).await
            }
            BrainRequestType::GetBrain => {
                BrainService::get(&scope, &serde_json::from_str(params)?).await
            }
            BrainRequestType::ListBrains => {
                BrainService::list(&scope, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
