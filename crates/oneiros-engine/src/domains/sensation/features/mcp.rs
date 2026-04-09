use crate::*;

pub struct SensationTools;

impl SensationTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        sensation_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        sensation_mcp::dispatch(context, tool_name, params).await
    }
}

mod sensation_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetSensation>::new(
                SensationRequestType::SetSensation,
                "Define a quality of connection between thoughts",
            )
            .def(),
            Tool::<GetSensation>::new(
                SensationRequestType::GetSensation,
                "Look up an experience category",
            )
            .def(),
            Tool::<ListSensations>::new(
                SensationRequestType::ListSensations,
                "See all the ways experiences can feel",
            )
            .def(),
            Tool::<RemoveSensation>::new(
                SensationRequestType::RemoveSensation,
                "Remove an experience category",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: SensationRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            SensationRequestType::SetSensation => {
                SensationService::set(context, &serde_json::from_str(params)?).await
            }
            SensationRequestType::GetSensation => {
                SensationService::get(context, &serde_json::from_str(params)?).await
            }
            SensationRequestType::ListSensations => {
                SensationService::list(context, &serde_json::from_str(params)?).await
            }
            SensationRequestType::RemoveSensation => {
                SensationService::remove(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
