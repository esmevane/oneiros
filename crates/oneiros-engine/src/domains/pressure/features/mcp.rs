use crate::*;

pub struct PressureTools;

impl PressureTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        pressure_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        pressure_mcp::dispatch(context, tool_name, params).await
    }
}

mod pressure_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<GetPressure>::new(
                PressureRequestType::GetPressure,
                "Check an agent's cognitive pressure",
            )
            .def(),
            Tool::<serde_json::Value>::new(
                PressureRequestType::ListPressures,
                "See all pressure readings",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: PressureRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            PressureRequestType::GetPressure => {
                PressureService::get(context, &serde_json::from_str(params)?).await
            }
            PressureRequestType::ListPressures => PressureService::list(context).await,
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
