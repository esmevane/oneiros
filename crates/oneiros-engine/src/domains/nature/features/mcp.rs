use crate::*;

pub struct NatureTools;

impl NatureTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        nature_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        nature_mcp::dispatch(context, tool_name, params).await
    }
}

mod nature_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetNature>::new(
                NatureRequestType::SetNature,
                "Define a kind of relationship between things",
            )
            .def(),
            Tool::<GetNature>::new(
                NatureRequestType::GetNature,
                "Look up a relationship category",
            )
            .def(),
            Tool::<ListNatures>::new(
                NatureRequestType::ListNatures,
                "See all the kinds of relationships",
            )
            .def(),
            Tool::<RemoveNature>::new(
                NatureRequestType::RemoveNature,
                "Remove a relationship category",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: NatureRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            NatureRequestType::SetNature => {
                NatureService::set(context, &serde_json::from_str(params)?).await
            }
            NatureRequestType::GetNature => {
                NatureService::get(context, &serde_json::from_str(params)?).await
            }
            NatureRequestType::ListNatures => {
                NatureService::list(context, &serde_json::from_str(params)?).await
            }
            NatureRequestType::RemoveNature => {
                NatureService::remove(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
