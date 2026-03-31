use crate::*;

pub struct PressureTools;

impl PressureTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        pressure_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        pressure_mcp::tool_names()
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

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "get_pressure",
                description: "Check an agent's cognitive pressure",
                input_schema: schema_for::<GetPressure>,
            },
            ToolDef {
                name: "list_pressures",
                description: "See all pressure readings",
                input_schema: schema_for::<serde_json::Value>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["get_pressure", "list_pressures"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "get_pressure" => PressureService::get(context, &serde_json::from_str(params)?).await,
            "list_pressures" => PressureService::list(context).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
