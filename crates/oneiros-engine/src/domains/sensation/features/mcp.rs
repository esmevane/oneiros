use crate::*;

pub struct SensationTools;

impl SensationTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        sensation_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        sensation_mcp::tool_names()
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

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_sensation",
                description: "Define a quality of connection between thoughts",
                input_schema: schema_for::<SetSensation>,
            },
            ToolDef {
                name: "get_sensation",
                description: "Look up an experience category",
                input_schema: schema_for::<GetSensation>,
            },
            ToolDef {
                name: "list_sensations",
                description: "See all the ways experiences can feel",
                input_schema: schema_for::<ListSensations>,
            },
            ToolDef {
                name: "remove_sensation",
                description: "Remove an experience category",
                input_schema: schema_for::<RemoveSensation>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &[
            "set_sensation",
            "get_sensation",
            "list_sensations",
            "remove_sensation",
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "set_sensation" => SensationService::set(context, &serde_json::from_str(params)?).await,
            "get_sensation" => SensationService::get(context, &serde_json::from_str(params)?).await,
            "list_sensations" => {
                SensationService::list(context, &serde_json::from_str(params)?).await
            }
            "remove_sensation" => {
                SensationService::remove(context, &serde_json::from_str(params)?).await
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
