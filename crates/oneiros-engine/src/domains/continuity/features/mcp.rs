use crate::*;

pub struct ContinuityTools;

impl ContinuityTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        continuity_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        continuity_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        continuity_mcp::dispatch(context, tool_name, params).await
    }
}

mod continuity_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "dream",
                description: "Restore an agent's full identity and cognitive context",
                input_schema: schema_for::<DreamAgent>,
            },
            ToolDef {
                name: "introspect",
                description: "Look inward before context compacts",
                input_schema: schema_for::<IntrospectAgent>,
            },
            ToolDef {
                name: "reflect",
                description: "Pause on something significant",
                input_schema: schema_for::<ReflectAgent>,
            },
            ToolDef {
                name: "sense",
                description: "Receive and interpret something from outside",
                input_schema: schema_for::<SenseContent>,
            },
            ToolDef {
                name: "sleep",
                description: "End a session — capture continuity before resting",
                input_schema: schema_for::<SleepAgent>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["dream", "introspect", "reflect", "sense", "sleep"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let no_overrides = DreamOverrides::default();

        let value = match tool_name {
            "dream" => {
                ContinuityService::dream(context, &serde_json::from_str(params)?, &no_overrides)
                    .await
            }
            "introspect" => {
                ContinuityService::introspect(
                    context,
                    &serde_json::from_str(params)?,
                    &no_overrides,
                )
                .await
            }
            "reflect" => {
                ContinuityService::reflect(context, &serde_json::from_str(params)?, &no_overrides)
                    .await
            }
            "sense" => {
                ContinuityService::sense(context, &serde_json::from_str(params)?, &no_overrides)
                    .await
            }
            "sleep" => {
                ContinuityService::sleep(context, &serde_json::from_str(params)?, &no_overrides)
                    .await
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
